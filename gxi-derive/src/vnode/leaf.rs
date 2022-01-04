use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

use crate::node::Node;

use super::VNodeDefault;

pub struct LeafVNode;

impl LeafVNode {
    pub fn parse_derive(input: TokenStream) -> TokenStream2 {
        let input = syn::parse::<syn::DeriveInput>(input).unwrap();
        let name = input.ident.to_token_stream();

        let vnode_default = VNodeDefault::derive(&name, &Node::Leaf.to_token_stream());

        quote! {
            impl gxi::VWidget for #name {}
            #vnode_default
        }
    }
}
