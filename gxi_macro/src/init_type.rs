use quote::__private::TokenStream;
use quote::*;

pub enum InitType {
    Child,
    Sibling,
}

impl ToTokens for InitType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            InitType::Child => tokens.append_all(quote! { InitType::Child }),
            InitType::Sibling => tokens.append_all(quote! { InitType::Sibling }),
        }
    }
}
