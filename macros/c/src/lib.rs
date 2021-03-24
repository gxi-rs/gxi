use proc_macro::TokenStream;

use crate::c_parser::CParser;

mod c_parser;

#[proc_macro]
pub fn c(item: TokenStream) -> TokenStream {
    let CParser { tree } = syn::parse_macro_input!(item as CParser);
    tree.into()
}
