mod iff_parser;

use proc_macro::TokenStream;

use crate::iff_parser::IffParser;

#[proc_macro]
pub fn iff(item: TokenStream) -> TokenStream {
    let IffParser { tree } = syn::parse_macro_input!(item as IffParser);
    tree.into()
}
