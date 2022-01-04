use crate::vnode::{ContainerVNode, LeafVNode};
use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;

pub enum Node {
    Leaf,
    Container,
    TopLevelContainer,
}

impl ToTokens for Node {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(match self {
            Node::Leaf => quote! { Leaf },
            Node::Container => quote! { Container },
            Node::TopLevelContainer => quote! { TopLevelContainer },
        });
    }
}

impl Node {
    pub fn parse_derive(&self, input: TokenStream) -> TokenStream {
        match self {
            Node::Leaf => LeafVNode::parse_derive(input),
            _ => ContainerVNode::parse_derive(input, self),
        }
        .into()
    }
}
