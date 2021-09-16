use proc_macro::TokenStream;

mod block;
mod blocks;
mod component;
mod execution;
mod init_type;

use quote::quote;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    let block = syn::parse_macro_input!(input as block::Block);
    (quote! {
        let __node = this.clone();
        #block
    })
    .into()
}
