use quote::*;
use syn::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};

use crate::TreeParser;

/// Parser for the [gxi_macro macro](../../gxi_macro/macro.gxi_macro.html).
pub struct GxiParser {
    pub tree: TokenStream2,
}

/// initialises the component
#[doc(hidden)]
#[macro_export]
macro_rules! comp_new {
    ($state_name:ident $state_cell:ident $state_cell_inner:ident $parent:ident { $($sender:tt)* } { $($p:ident : $t:ty = $v:expr)* }) => {
        Rc::new(RefCell::new(Box::new(Self {
            state: $state_cell::new($state_cell_inner::new($state_name {
                $($p:$v),*
            })),
            $($sender)*
            self_substitute : None,
            $parent,
            dirty: true,
            child: None,
            sibling: None,
        })));
    };
}

/// creates the State Struct
#[doc(hidden)]
#[macro_export]
macro_rules! comp_state {
    ($state_name:ident { $($p:ident : $t:ty = $v:expr)* }) => {
        pub struct $state_name {
            $(pub $p:$t),*
        }
    };
}

/// for Arc<Mutex<State>>>
#[doc(hidden)]
#[macro_export]
macro_rules! arc_state_unwrap {
    ($state:ident) => {
        $state.lock().unwrap();
    };
}

/// for Arc<Mutex<State>>>
#[doc(hidden)]
#[macro_export]
macro_rules! rc_state_unwrap {
    ($state:ident) => {
        $state.borrow_mut();
    };
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
            let state_cloner = quote! {
                let state = {
                    let state_borrow = this.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                    state.state.clone()
                };
            };
            let update_inner = if !is_async {
                quote! {
                #state_cloner
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
                if let ShouldRender::Yes = update(state,msg,render).unwrap() {
                    {
                        let mut node = this.as_ref().borrow_mut();
                        node.mark_dirty();
                    }
                    Self::render(this);
                }
            }
            } else if cfg!(feature = "desktop") {
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
                quote! {
                #state_cloner
                spawn_local(async move {
                    let render =  {
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
                    //gxi_update_macro logic. Made to return should render to force dev to decide render state
                    #update_fn
                    if let ShouldRender::Yes = update(state,msg,render).await.unwrap() {
                        {
                            let mut node = this.as_ref().borrow_mut();
                            node.mark_dirty();
                        }
                        Self::render(this);
                    }
                });
            }
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
    /// parses the `input` parse_steam according to the syntax defined in the [gxi_macro macro](../../gxi_macro/macro.gxi_macro.html#syntax)
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
                                let node = cont.clone();
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
        let (state_cell, state_cell_inner, import_get_state_macro) = {
            if is_update_async && cfg!(feature = "desktop") {
                (
                    quote!(Arc),
                    quote!(Mutex),
                    quote!(gxi::arc_state_unwrap)
                )
            } else {
                (
                    quote!(Rc),
                    quote!(RefCell),
                    quote!(gxi::rc_state_unwrap)
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
