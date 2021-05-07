use quote::*;
use syn::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};

use crate::TreeParser;

/// Parser for the [macros macro](../../macros/macro.macros.html).
pub struct GxiParser {
    pub tree: TokenStream2,
}

impl GxiParser {
    fn parse_update_fn(name: &Ident, update_block: Block, is_async: bool) -> TokenStream2 {
        let async_ident = if is_async { quote!(async) } else { TokenStream2::new() };
        let update_fn = quote! {
            #async_ident
            fn update<F: Fn() + 'static>(
                state: State, msg: Msg, render: F) -> AsyncResult<ShouldRender>
                #update_block
        };
        let update_inner = {
            let update_inner = if is_async && cfg!(feature = "desktop") {
                quote! {
                    let (channel_sender, state) = {
                        let state_borrow = this.as_ref().borrow();
                        let state = state_borrow.as_any().downcast_ref::<#name>().unwrap();
                        (state.channel_sender.clone(), state.state.clone())
                    };
                    tokio::task::spawn(async move {
                        let render = {
                            let channel_sender = channel_sender.clone();
                            move || channel_sender.send(()).unwrap()
                        };
                        //gxi_update_macro logic. Made to return should render to force dev to decide render state
                        #update_fn
                        if let ShouldRender::Yes = update(state,msg,render).await.unwrap() {
                            channel_sender.send(()).unwrap()
                        }
                    });
                }
            } else {
                let await_call = if is_async {
                    quote!(.await)
                } else {
                    TokenStream2::new()
                };
                let mut update_inner = quote! {
                    let state = {
                        let state_borrow = this.as_ref().borrow();
                        let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                        state.state.clone()
                    };
                    let render = {
                        let this = Rc::clone(&this);
                        move || {
                            let this = Rc::clone(&this);
                            {
                                let mut node = this.as_ref().borrow_mut();
                                node.mark_dirty();
                            }
                            Self::render(this);
                        }
                    };
                    #update_fn
                    if let ShouldRender::Yes = update(state,msg,render)#await_call.unwrap() {
                        {
                            let mut node = this.as_ref().borrow_mut();
                            node.mark_dirty();
                        }
                        Self::render(this);
                    }
                };
                if cfg!(feature = "web") {
                    update_inner = quote! {
                        spawn_local(async move {
                            #update_inner
                        });
                    }
                }
                update_inner
            };
            update_inner
        };
        quote! {
            impl #name {
                fn update(this: NodeRc, msg: Msg) {
                    #update_inner
                }
            }
        }
    }
}

impl Parse for GxiParser {
    /// parses the `input` parse_steam according to the syntax defined in the [macros macro](../../macros/macro.macros.html#syntax)
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let state_name = syn::Ident::new(&format!("{}State", quote! {#name}), Span::call_site());
        let state_block = input.parse::<syn::Block>()?;
        let mut render_func = TokenStream2::new();
        let mut update_func = TokenStream2::new();
        let mut is_update_async = false;
        for _ in 0..2 {
            if let Ok(s) = input.parse::<syn::Ident>() {
                match &s.to_string()[..] {
                    "render" => {
                        let block_content = group::parse_braces(&input)?.content;
                        let content = TreeParser::parse(&block_content)?.tree;
                        render_func = quote!(
                            fn render(this: NodeRc) {
                                let cont = Rc::clone(&this);
                                let state = {
                                    let mut node_borrow = this.as_ref().borrow_mut();
                                    let node = node_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                                    if !node.is_dirty() {
                                        return;
                                    }
                                    node.mark_clean();
                                    node.state.clone()
                                };
                                let state = get_state!(state);
                                #content
                            }
                        );
                    }
                    "update" => {
                        if let Ok(async_ident) = input.parse::<syn::Ident>() {
                            if async_ident != "async" {
                                return Err(syn::Error::new(async_ident.span(), "expected async here"));
                            }
                            is_update_async = true;
                        }
                        update_func = GxiParser::parse_update_fn(&name, input.parse::<syn::Block>()?, is_update_async);
                    }
                    _ => return Err(syn::Error::new(s.span(), "Didn't expect this attribute here"))
                }
            }
        }

        // need not use Arc<Mutex<>> in web and when update is not async
        let (state_cell, state_cell_inner, import_get_state_macro, import_get_state_macro_mut) = {
            if is_update_async && cfg!(feature = "desktop") {
                (
                    quote!(Arc),
                    quote!(Mutex),
                    quote!(gxi::get_arc_state),
                    quote!(gxi::get_arc_state),
                )
            } else {
                (
                    quote!(Rc),
                    quote!(RefCell),
                    quote!(gxi::get_rc_state),
                    quote!(gxi::get_mut_rc_state),
                )
            }
        };

        let (desktop_channel_new, sender_field, sender_struct_field, channel_attach) =
            if cfg!(feature = "desktop") && is_update_async {
                (
                    quote! { let (channel_sender, re) = glib::MainContext::channel(glib::PRIORITY_DEFAULT); },
                    quote! { channel_sender, },
                    quote! { pub channel_sender: glib::Sender<()>, },
                    quote! {{
                        let this = this.clone();
                        re.attach(None, move |_| {
                            let this = Rc::clone(&this);
                            //mark dirty
                            {
                                let mut node = this.as_ref().borrow_mut();
                                node.mark_dirty();
                            }
                            Self::render(this);
                            glib::Continue(true)
                        });
                    }}
                )
            } else {
                (
                    TokenStream2::new(),
                    TokenStream2::new(),
                    TokenStream2::new(),
                    TokenStream2::new(),
                )
            };

        Ok(GxiParser {
            tree: quote! {
                use #import_get_state_macro as get_state;
                use #import_get_state_macro_mut as get_state_mut;
                use std::any::Any;
                use std::borrow::Borrow;
                use std::cell::RefCell;
                use std::rc::Rc;
                use std::sync::{Mutex, Arc};

                type State = #state_cell<#state_cell_inner<#state_name>>;

                comp_state!(#state_name #state_block);

                pub struct #name {
                    pub state: State,
                    #sender_struct_field
                    pub parent: WeakNodeRc,
                    pub self_substitute : Option<WeakNodeRc>,
                    pub dirty: bool,
                    pub child: Option<NodeRc>,
                    pub sibling: Option<NodeRc>
                }

                impl Node for #name {
                    impl_node_for_component!();

                    fn new(parent: WeakNodeRc) -> NodeRc {
                        #desktop_channel_new
                        let this:NodeRc = comp_new!(#state_name #state_cell #state_cell_inner parent { #sender_field } #state_block );
                        {
                            let mut this_borrow = this.as_ref().borrow_mut();
                            let this_borrow = this_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                            this_borrow.self_substitute = Some(Rc::downgrade(&this));
                        }
                        #channel_attach
                        this
                    }

                    #render_func
                }

                #update_func

                impl_drop_for_component!(#name);
            },
        })
    }
}
