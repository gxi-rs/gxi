use quote::*;
use syn::__private::TokenStream2;

pub enum InitType {
    Child(u32),
    Sibling(u32),
}

impl InitType {
    /// Return: [PureIndex](PureIndex) and [InitType](InitType)
    pub fn get_init_type_tuple(&self) -> (u32, TokenStream2) {
        match self {
            InitType::Child(i) => (*i, quote! {init_child}),
            InitType::Sibling(i) => (*i, quote! {init_sibling}),
        }
    }
}
