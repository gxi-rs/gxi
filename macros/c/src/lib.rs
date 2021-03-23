use proc_macro::TokenStream;

use quote::quote;
use syn::{Block, Error, Expr, Ident, Stmt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

struct Combinations {}

impl Parse for Combinations {
    fn parse(input: ParseStream) -> Result<Self, Error> {

        Ok(Combinations {})
    }
}

#[proc_macro]
pub fn c(item: TokenStream) -> TokenStream {
    let Combinations { .. } = syn::parse_macro_input!(item as Combinations);

    (quote! {
        fn render(container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
            let cont = Rc::clone(&container);
            let node = cont.clone();

        }
    }).into()
}