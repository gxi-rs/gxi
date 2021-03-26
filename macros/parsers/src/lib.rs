mod n_parser;
mod x_parser;
mod c_parser;

pub use n_parser::*;
pub use c_parser::*;
pub use x_parser::*;
use syn::__private::TokenStream2;

pub enum InitType {
    Child,
    Sibling,
    Pure(u32),
}
impl InitType {
    fn get_init_quote(&self) -> TokenStream2{
        match self {
            InitType::Child => NParser::parse(quote! { #name init_child #block }.into()).unwrap().tree,
            InitType::Sibling => (quote! { n!(#name init_sibling #block); }),
            InitType::Pure(i) => quote! { n!(#i #name init_child #block); }
        }
    }
}