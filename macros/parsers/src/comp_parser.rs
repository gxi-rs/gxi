use quote::*;
use syn::*;
use syn::__private::*;
use syn::parse::{Parse, ParseStream};

use crate::TreeParser;

pub struct CompParser {
    pub tree: TokenStream2,
}

#[macro_export]
macro_rules! comp_init {
    ($name:ident { $($p:ident : $t:ty = $v:expr);* } { $($render:tt)* } { $($update:tt)* } )=> {
        use std::any::Any;
        use std::borrow::Borrow;
        use std::cell::RefCell;
        use std::rc::Rc;

        pub struct $name {
            pub $($p:$t),*,
            pub parent: WeakNodeRc,
            pub dirty: bool,
            pub child: Option<NodeRc>,
            pub sibling: Option<NodeRc>,
            pub widget: gtk::Container,
        }

        impl Node for $name {
            impl_node_for_component!();

            fn new(parent: WeakNodeRc) -> NodeRc {
                Rc::new(RefCell::new(Box::new(Self {
                    $($p:$v),*,
                    widget: parent.upgrade().unwrap().as_ref().borrow().get_widget_as_container(),
                    parent,
                    dirty: true,
                    child: None,
                    sibling: None,
                })))
            }
            $($render)*
        }

        impl $name {
            $($update)*
        }

        impl_drop_for_component!($name);
    };
}

impl Parse for CompParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let props = input.parse::<syn::Block>()?;
        let mut render_func = quote!(
            fn render(_top_state: NodeRc) {}
        );
        let mut update_func = quote!(
            fn update(state: NodeRc, msg: Msg) -> ShouldRender {
                ShouldRender::No
            }
        );
        for _ in 0..2 {
            if let Ok(s) = input.parse::<syn::Ident>() {
                match &s.to_string()[..] {
                    "render" => {
                        let block_content = group::parse_braces(&input)?.content;
                        let content = TreeParser::parse(&block_content)?.tree;
                        render_func = quote!(
                            fn render(top_state: NodeRc) {
                                let cont = Rc::clone(&top_state);
                                let node = cont.clone();
                                #content
                            }
                        );
                    }
                    "update" => {
                        let block = input.parse::<syn::Block>()?;
                        update_func = quote! {
                            fn update(state: NodeRc, msg: Msg) -> ShouldRender {
                                let mut state_borrow = state.as_ref().borrow_mut();
                                let state = state_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                                #block
                            }
                        }
                    }
                    _ => panic!(""),
                }
            }
        }
        Ok(CompParser {
            tree: quote!(comp_init!(#name #props {#render_func} {#update_func});),
        })
    }
}
