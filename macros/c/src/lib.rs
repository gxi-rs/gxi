mod first_child;
mod nth_child;

use proc_macro::TokenStream;
use crate::first_child::FirstChild;

#[proc_macro]
pub fn c(item: TokenStream) -> TokenStream {
    let FirstChild { tree } = syn::parse_macro_input!(item as FirstChild);
    tree.into()
}
