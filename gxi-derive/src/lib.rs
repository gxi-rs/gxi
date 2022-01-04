extern crate proc_macro;

use node::Node;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use vnode::VNodeDefault;

mod native_node;
mod node;
mod vnode;

#[proc_macro_attribute]
pub fn gxi_vnode(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse::<syn::ItemStruct>(input).unwrap();
    let name = input.ident.to_token_stream();
    let derive = VNodeDefault::derive(&name, &attr.into());
    (quote! {
        #input
        #derive
    })
    .into()
}

#[proc_macro_derive(LeafNode)]
pub fn gxi_leaf_node_derive(input: TokenStream) -> TokenStream {
    Node::Leaf.parse_derive(input).into()
}

#[proc_macro_derive(TopLevelContainerNode)]
pub fn gxi_top_level_container_node_derive(input: TokenStream) -> TokenStream {
    Node::TopLevelContainer.parse_derive(input).into()
}

#[proc_macro_derive(ContainerNode)]
pub fn gxi_container_node_derive(input: TokenStream) -> TokenStream {
    Node::Container.parse_derive(input).into()
}
