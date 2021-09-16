use quote::__private::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Clone)]
pub enum InitType {
    Child(bool),
    Sibling,
}

impl ToTokens for InitType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            InitType::Child(add_to_self_substitute) => tokens.append_all(quote! {InitType::Child(#add_to_self_substitute)}),
            InitType::Sibling => tokens.append_all(quote! {InitType::Sibling(&__cont)}),
        }
    }
}
