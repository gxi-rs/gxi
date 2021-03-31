use std::borrow::Borrow;

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
    ($name:ident { $($p:ident : $t:ty = $v:literal)? } { $($render:tt)* } { $($update:tt)* } )=> {
        pub struct $name {
            pub $($p:$t),*,
            pub child: Option<AsyncNode>,
            pub sibling: Option<AsyncNode>,
            pub widget: gtk::Container,
        }

        impl Node for $name {
            impl_node_component!();

            fn new(parent_widget: Option<gtk::Container>) -> AsyncNode {
                Rc::new(RefCell::new(Box::new(Self {
                    $($p:$v),*,
                    child: None,
                    sibling: None,
                    widget: parent_widget.unwrap(),
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
            fn render(_top_state: AsyncNode) {}
        );
        let mut update_func = quote!(
            fn update(state: AsyncNode, msg: Msg) -> ShouldRender {ShouldRender::No}
        );
        for _ in 0..2 {
            if let Ok(s) = input.parse::<syn::Ident>() {
                match &s.to_string()[..] {
                    "render" => {
                        let block_content = group::parse_braces(&input)?.content;
                        let content = TreeParser::parse(&block_content)?.tree;
                        render_func = quote!(
                            fn render(top_state: AsyncNode) {
                                let cont = Rc::clone(&top_state);
                                let node = cont.clone();
                                #content
                            }
                        );
                    }
                    "update" => {
                        let block = input.parse::<syn::Block>()?;
                        update_func = quote! {
                            fn update(state: AsyncNode, msg: Msg) -> ShouldRender {
                                let mut state_borrow = state.as_ref().borrow_mut();
                                let state = state_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                                #block
                            }
                        }
                    }
                    _ => panic!("")
                }
            }
        }
        Ok(CompParser { tree: quote!(comp_init!(#name #props {#render_func} {#update_func});) })
    }
}

/*
{
            let mut props = vec![];
            fn parse_struct(content: ParseBuffer, vec: &mut Vec<(TokenStream2, TokenStream2, TokenStream2)>) -> Result<()> {
                let prop = content.parse::<syn::Ident>()?;
                let prop_type = content.parse::<syn::Type>()?;
                content.parse::<syn::token::Eq>()?;
                let value = content.parse::<syn::Lit>()?;
                vec.push(prop)
            }
            {
                let brackets = group::parse_brackets(&input)?;
                let tree = parse_struct(brackets.content, &mut props)?;
                quote! {
                    struct #name {
                        #struct_tree
                    }
                }
            }
        }
*/
