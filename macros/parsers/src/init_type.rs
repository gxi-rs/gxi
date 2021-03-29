use quote::*;
use syn::__private::TokenStream2;

pub enum InitType {
    Child,
    Sibling,
    PureChild(u32),
    PureSibling(u32)
}

impl InitType {
    pub fn get_init_type_tuple(&self) -> (u32, TokenStream2) {
        match self {
            InitType::Child => (0, quote! {init_child}),
            InitType::PureSibling(i) => (*i, quote! {init_sibling}),
            InitType::PureChild(i) => (*i, quote! {init_child}),
            InitType::Sibling => (0, quote! {init_sibling}),
        }
    }
}
