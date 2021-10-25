use quote::__private::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

pub enum VNodeType {
    Widget,
    ContainerWidget,
    TopLevelContainerWidget,
}

impl ToTokens for VNodeType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(match self {
            VNodeType::Widget => quote! { Widget },
            VNodeType::ContainerWidget => quote! { ContainerWidget },
            VNodeType::TopLevelContainerWidget => quote! { TopLevelContainerWidget },
        });
    }
}
