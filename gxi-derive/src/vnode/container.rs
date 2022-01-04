use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

use super::VNodeDefault;
use crate::node::Node;

pub struct ContainerVNode;

impl ContainerVNode {
    pub fn parse_derive(input: TokenStream, node: &Node) -> TokenStream2 {
        let input = syn::parse::<syn::DeriveInput>(input).unwrap();
        let name = input.ident.to_token_stream();

        let v_node_default = VNodeDefault::derive(&name, &Node::Leaf.to_token_stream());

        quote! {
            impl gxi::VContainerWidget for #name {}
            #v_node_default
        }
    }
}
