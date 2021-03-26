use parsers::CParser;
use proc_macro::TokenStream;

#[proc_macro]
pub fn x(item: TokenStream) -> TokenStream {
    let CParser { tree } = syn::parse_macro_input!(item as CParser);
    tree.into()
}
