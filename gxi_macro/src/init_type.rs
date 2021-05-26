use quote::*;
use syn::__private::{Span};
use quote::__private::TokenStream;

pub enum InitType {
    Child,
    Sibling,
}

impl ToTokens for InitType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            InitType::Child => tokens.append(syn::Ident::new("init_child", Span::call_site())),
            InitType::Sibling => tokens.append(syn::Ident::new("init_sibling", Span::call_site())),
        }
    }
}
