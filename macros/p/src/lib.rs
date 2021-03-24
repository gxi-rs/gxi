use proc_macro::TokenStream;

use crate::p_parser::PParser;

mod p_parser;

#[proc_macro]
pub fn p(item: TokenStream) -> TokenStream {
    let PParser { tree } = syn::parse_macro_input!(item as PParser);
    tree.into()
}

