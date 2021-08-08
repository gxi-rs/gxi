use proc_macro::TokenStream;

use crate::tree_parser::TreeParser;

mod component;
mod init_type;
mod tree_parser;

#[doc = include_str!("../README.md")]
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    let TreeParser(tree) = syn::parse_macro_input!(input as TreeParser);
    tree.into()
}
