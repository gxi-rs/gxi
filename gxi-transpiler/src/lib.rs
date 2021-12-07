use proc_macro::TokenStream;
use syn::__private::TokenStream2;

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
    let block = syn::parse_macro_input!(input as block::Block);
    let mut tokens = TokenStream2::new();
    block.to_tokens(&mut tokens, 0);
    tokens.into()
}
