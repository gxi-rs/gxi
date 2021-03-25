mod forr_parser;

use proc_macro::TokenStream;

use crate::forr_parser::ForrParser;


#[proc_macro]
pub fn forr(item: TokenStream) -> TokenStream {
    let ForrParser { tree } = syn::parse_macro_input!(item as ForrParser);
    tree.into()
}

