use proc_macro::TokenStream;

mod block;
mod blocks;
mod component;
mod execution;
mod conditional;
mod scope;
mod optional_parse;

use quote::ToTokens;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    let block = syn::parse_macro_input!(input as block::Block);
    block.to_token_stream().into()
}
