use proc_macro::TokenStream;

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
    block.to_token_stream(Default::default()).into()
}
