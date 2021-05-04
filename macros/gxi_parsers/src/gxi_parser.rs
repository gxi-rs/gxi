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
    ($state_name:ident $parent:ident { $($sender:tt)* } { $($p:ident : $t:ty = $v:expr)* }) => {
        Rc::new(RefCell::new(Box::new(Self {
            state: Arc::new(Mutex::new($state_name {
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

impl Parse for GxiParser {
    /// parses the `input` parse_steam according to the syntax defined in the [gxi_macro macro](../../gxi_macro/macro.gxi_macro.html#syntax)
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let state_name = syn::Ident::new(&format!("{}State", quote! {#name}), Span::call_site());
        let state_block = input.parse::<syn::Block>()?;
        let mut render_func = TokenStream2::new();
        let mut update_func = TokenStream2::new();

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
                                let state = state.lock().unwrap();
                                #content
                            }
                        );
                    }
                    "update" => {
                        let async_ident =
                            if let Ok(async_ident) = input.parse::<syn::Ident>() {
                                if async_ident != "async" {
                                    return Err(syn::Error::new(async_ident.span(), "expected async here"));
                                }
                                quote!(#async_ident)
                            } else { TokenStream2::new() };
                        let content = input.parse::<syn::Block>()?;
                        update_func = quote!(
                            #[update(#name)]
                            #async_ident fn update<F: Fn() + 'static>(state: AsyncState, msg: Msg, render: F) -> AsyncResult<ShouldRender>
                                #content
                        );
                    }
                    _ => return Err(syn::Error::new(s.span(), "Didn't expect this attribute here"))
                }
            }
        }

        #[cfg(feature = "desktop")]
            let (desktop_channel_new, sender_field, sender_struct_field, channel_attach) = (
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
        );

        #[cfg(feature = "web")]
            let (desktop_channel_new, sender_field, sender_struct_field, channel_attach) = (
            TokenStream2::new(),
            TokenStream2::new(),
            TokenStream2::new(),
            TokenStream2::new(),
        );

        Ok(GxiParser {
            tree: quote! {
                use std::any::Any;
                use std::borrow::Borrow;
                use std::cell::RefCell;
                use std::rc::Rc;
                use std::sync::{Mutex, Arc};

                type AsyncState = Arc<Mutex<#state_name>>;

                comp_state!(#state_name #state_block);

                pub struct #name {
                    pub state: AsyncState,
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
                        let this:NodeRc = comp_new!(#state_name parent { #sender_field } #state_block );
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
