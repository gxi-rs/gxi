use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::TokenStream;

pub enum VNodeType {
    Component,
    Widget,
    ContainerWidget,
    TopLevelContainerWidget,
    TopLevelContainer,
}

impl ToTokens for VNodeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            VNodeType::Component => quote! { Component },
            VNodeType::Widget => quote! { Widget },
            VNodeType::ContainerWidget => quote! { ContainerWidget },
            VNodeType::TopLevelContainerWidget => quote! { TopLevelContainerWidget },
            VNodeType::TopLevelContainer => quote! { TopLevelContainer },
        });
    }
}
