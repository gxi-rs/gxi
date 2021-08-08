use syn::{__private::TokenStream2, parse::Parse};

pub struct TreeParser(pub TokenStream2);

impl Parse for TreeParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}
