use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::TokenStream;

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
