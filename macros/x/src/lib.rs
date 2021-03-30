mod x_parser;

use parsers::TreeParser;
use proc_macro::TokenStream;

#[proc_macro]
pub fn x(item: TokenStream) -> TokenStream {
    let TreeParser { tree } = syn::parse_macro_input!(item as TreeParser);
    tree.into()
}
