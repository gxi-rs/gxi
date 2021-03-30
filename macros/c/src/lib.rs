use proc_macro::TokenStream;

use parsers::TreeParser;

#[proc_macro]
pub fn c(item: TokenStream) -> TokenStream {
    let TreeParser { tree } = syn::parse_macro_input!(item as TreeParser);
    tree.into()
}
