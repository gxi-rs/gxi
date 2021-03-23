use proc_macro::TokenStream;

use quote::quote;
use syn::{Block, Error, Ident};
use syn::parse::{Parse, ParseBuffer, Parser, ParseStream};

struct Combinations {
    name: Ident,
    block: Block,
}

impl Parse for Combinations {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let name = input.parse()?;
        let block = input.parse()?;
        Ok(Combinations { name, block })
    }
}

#[proc_macro]
pub fn proc_node(item: TokenStream) -> TokenStream {
    let name = syn::parse_macro_input!(item as Combinations);
    let name = name.name;
    let result = quote! {
        let node = {
            let mut node_borrow = node.as_ref().borrow_mut();
            let node = Rc::clone(&node);
            node_borrow.init_child(Box::new(move || #name::new(node.clone())), true).0
        };
    };
    result.into()
}
/*
#[proc_macro]
pub fn proc_node(item: TokenStream) -> TokenStream {
    let name= syn::parse_macro_input!(item as syn::Ident);
    let name = &name;
    let result = quote! {
        {
            let (node, _is_new) = {
                let mut node_borrow = $prev_node.as_ref().borrow_mut();
                let container = Rc::clone(&$prev_node);
                node_borrow.$type(Box::new(move || $Node::new(container.clone())), true)
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<$Node>().unwrap();
                let state = $state.as_ref().borrow();
                $(
                    node.widget.$f(state.$v.to_string().as_str());
                )*
            }
            node
        }
    };
    result.into()
}
*/