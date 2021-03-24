mod first_child;
mod nth_child;

use proc_macro::TokenStream;

use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::group::Brackets;
use syn::parse::{Parse, ParseStream};
use crate::first_child::FirstChild;

#[proc_macro]
pub fn c(item: TokenStream) -> TokenStream {
    let FirstChild { tree } = syn::parse_macro_input!(item as FirstChild);
    tree.into()
}
