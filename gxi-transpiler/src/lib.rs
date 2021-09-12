use proc_macro::TokenStream;

mod block;
mod blocks;
mod component;
mod execution;
mod init_type;

pub(crate) use block::*;
pub(crate) use blocks::*;
pub(crate) use component::*;
pub(crate) use execution::*;
pub(crate) use init_type::*;
use quote::ToTokens;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    let block = syn::parse_macro_input!(input as Block);
    block.into_token_stream().into()
}
