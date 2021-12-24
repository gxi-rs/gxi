use proc_macro::TokenStream;
use quote::ToTokens;
use syn::__private::TokenStream2;

use crate::component::NodeBlock;

mod block;
mod blocks;
mod component;
mod conditional;
mod execution;
mod scope;
#[macro_use]
mod optional_parse;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    // Other blocks can't exist independently
    let block = syn::parse_macro_input!(input as NodeBlock);
    let mut tokens = TokenStream2::new();
    block.to_tokens(&mut tokens);
    tokens.into()
}
