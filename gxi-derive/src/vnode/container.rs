use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

use super::VNodeDefault;
use crate::{native_node::NativeNode, node::Node};

pub struct ContainerVNode;

impl ContainerVNode {
    pub fn parse_derive(input: TokenStream, node: &Node) -> TokenStream2 {
        let input = syn::parse::<syn::DeriveInput>(input).unwrap();
        let name = input.ident.to_token_stream();

        let v_node_default = VNodeDefault::derive(&name, &node.to_token_stream());
        let native_node_deref = NativeNode::Container.impl_deref(&name);

        quote! {
            impl gxi::VContainer for #name {}
            #v_node_default
            #native_node_deref
        }
    }
}
