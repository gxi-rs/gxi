use quote::*;
use syn::__private::TokenStream2;

pub enum InitType {
    Child,
    Sibling,
    Pure(u32),
}

impl InitType {
    pub fn get_init_quote(&self) -> (u32, TokenStream2) {
        match self {
            InitType::Child => (0, quote! {init_child}),
            InitType::Pure(i) => (*i, quote! {init_sibling}),
            InitType::Sibling => (0, quote! {init_sibling})
        }
    }
}