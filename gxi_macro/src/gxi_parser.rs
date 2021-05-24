use quote::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};
use syn::*;

use crate::TreeParser;

/// Parser for the [macros macro](../../macros/macro.macros.html).
pub struct GxiParser {
    pub tree: TokenStream2,
}

impl GxiParser {
    // parse `{}` brackets where state is defined
    fn parse_state_block(input: &ParseStream) -> Result<(TokenStream2, TokenStream2)> {
        let block = group::parse_braces(&input)?.content;
        // syntax -> field_name : type = value (optional) comma
        let mut state_struct_lines = vec![];
        let mut state_init_lines = vec![];
        {
            loop {
                let field_name = block.parse::<syn::Ident>()?;
                block.parse::<token::Colon>()?;
                let field_type = block.parse::<syn::Type>()?;
                // if equals-to is present, parse default value
                let field_value: TokenStream2 = if block.parse::<syn::token::Eq>().is_ok() {
                    let value = block.parse::<syn::Expr>()?;
                    quote! { #value }
                } else {
                    quote! {
                        Default::default()
                    }
                };

                state_struct_lines.push(
                    quote! {
                        #field_name : #field_type
                    }
                );

                state_init_lines.push(
                    quote! {
                        #field_name : #field_value
                    }
                );

                // break when , is not found
                if input.parse::<token::Comma>().is_err() {
                    break;
                }
            }
        }

        Ok((
            quote! {
                #( #state_struct_lines ),*
            },
            quote! {
                #( #state_init_lines ),*
            }
        ))
    }

    /// parse update block
    fn parse_update_fn(name: &Ident, input: &ParseStream, is_async: bool) -> Result<TokenStream2> {
        // create `async` ident if component is async
        let async_ident = if is_async {
            quote!(async)
        } else {
            TokenStream2::new()
        };
        // generate the update function
        let update_fn = {
            let update_block = input.parse::<syn::Block>()?;
            quote! {
                #async_ident
                fn update<F: Fn() + 'static>(
                    state: State, msg: Msg, render: F) -> AsyncResult<ShouldRender>
                    #update_block
            }
        };
        // inner logic for executing the update function
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

        Ok(quote! {
            impl #name {
                fn update(this: NodeRc, msg: Msg) {
                    #update_inner
                }
            }
        })
    }
}

impl Parse for GxiParser {
    /// parses the `input` parse_steam according to the syntax defined in the [macros macro](../../macros/macro.macros.html#syntax)
    fn parse(input: ParseStream) -> Result<Self> {
        let viz = input.parse::<syn::Visibility>().unwrap_or(Visibility::Inherited);
        // check if component is async
        let (name, is_async) = {
            // this ident can either be `async` or it could be the name of the component
            let ident = input.parse::<syn::Ident>()?;
            if ident == "async" {
                (input.parse::<syn::Ident>()?, true)
            } else {
                (ident, false)
            }
        };
        // name of the state made by concatenating name of the component to String `State`
        let state_name = syn::Ident::new(&format!("{}State", quote! {#name}), Span::call_site());
        // parse `{}` brackets where state is defined
        let (state_struct, state_new) = Self::parse_state_block(&input)?;
        // update and render function
        let mut render_func = TokenStream2::new();
        let mut update_func = TokenStream2::new();
        // parse blocks
        for _ in 0..2 {
            if let Ok(s) = input.parse::<syn::Ident>() {
                match &s.to_string()[..] {
                    "render" => {
                        let block_content = group::parse_braces(&input)?.content;
                        let content = TreeParser::parse(&block_content)?.tree;
                        render_func = quote! {
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
                        };
                    }
                    "update" => {
                        update_func = GxiParser::parse_update_fn(
                            &name,
                            &input,
                            is_async,
                        )?;
                    }
                    _ => {
                        return Err(syn::Error::new(
                            s.span(),
                            "Didn't expect this attribute here",
                        ));
                    }
                }
            }
        }

        // need not use Arc<Mutex<>> in web and when update is not async
        let (state_cell, state_cell_inner, import_get_state_macro, import_get_state_macro_mut) = {
            if is_async && cfg!(feature = "desktop") {
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

        let (desktop_channel_new, channel_sender_field, channel_sender_struct_field, desktop_channel_attach) = if cfg!(
            feature = "desktop"
        )
            && is_async
        {
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
                }},
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

                #viz struct #state_name {
                    #state_struct
                }

                #viz struct #name {
                    state: State,
                    #channel_sender_struct_field
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
                        // init
                        let this:NodeRc = Rc::new(RefCell::new(Box::new(Self {
                            state: #state_cell::new(#state_cell_inner::new(#state_name {
                                #state_new
                            })),
                            #channel_sender_field
                            self_substitute : None,
                            parent,
                            dirty: true,
                            child: None,
                            sibling: None,
                        })));
                        {
                            let mut this_borrow = this.as_ref().borrow_mut();
                            let this_borrow = this_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                            this_borrow.self_substitute = Some(Rc::downgrade(&this));
                        }
                        #desktop_channel_attach
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
