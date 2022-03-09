use proc_macro::TokenStream;
use quote::ToTokens;
use root_block::RootBlock;

mod observer_builder;
mod conditional;
mod execution;
mod node;
mod root_block;
mod scope;
mod sub_tree;
#[macro_use]
mod optional_parse;
mod lifetime;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(input as RootBlock)
        .to_token_stream()
        .into()
}
