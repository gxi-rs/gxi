use proc_macro::TokenStream;
use quote::ToTokens;
use syn::__private::TokenStream2;

use crate::node::NodeBlock;

mod conditional;
mod execution;
mod node;
mod scope;
mod sub_tree;
#[macro_use]
mod optional_parse;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    let block = syn::parse_macro_input!(input as NodeBlock);
    let mut tokens = TokenStream2::new();
    block.to_tokens(&mut tokens);
    tokens.into()
}
