use quote::__private::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

pub enum VNodeType {
    Component,
    Widget,
}

impl ToTokens for VNodeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            VNodeType::Component => tokens.append_all(quote! { Component }),
            VNodeType::Widget => tokens.append_all(quote! { Widget }),
        }
    }
}
