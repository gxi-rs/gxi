use proc_macro::TokenStream;

mod block;
mod component;
mod execution;
mod init_type;

pub(crate) use block::*;
pub(crate) use component::*;
pub(crate) use execution::*;
pub(crate) use init_type::*;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    let block_parser = syn::parse_macro_input!(input as BlockParser);
    block_parser.into()
}
