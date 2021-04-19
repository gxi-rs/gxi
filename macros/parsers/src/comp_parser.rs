use quote::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};
use syn::*;

use crate::TreeParser;

pub struct CompParser {
    pub tree: TokenStream2,
}

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

#[macro_export]
macro_rules! comp_state {
    ($state_name:ident { $($p:ident : $t:ty = $v:expr)* }) => {
        pub struct $state_name {
            $(pub $p:$t),*
        }
    };
}

impl Parse for CompParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let state_name = syn::Ident::new(&format!("{}State", quote! {#name}), Span::call_site());
        let state_block = input.parse::<syn::Block>()?;
        let mut render_func = TokenStream2::new();
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
                    _ => panic!("Didn't expect this attribute here"),
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

        Ok(CompParser {
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

                impl_drop_for_component!(#name);
            },
        })
    }
}
