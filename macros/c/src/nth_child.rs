use proc_macro::TokenStream;

use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::group::Brackets;
use syn::parse::{Parse, ParseStream};

pub struct NthChild {
    pub tree: TokenStream2
}

impl Parse for NthChild {
    fn parse(input: ParseStream) -> Result<Self> {
        let tree = {
            let name: Ident = input.parse()?;
            let block: Block = input.parse()?;
            quote! { n!(#name init_child #block); }
        };
        //children are optional
        if let Ok(block) = input.parse::<Block>() {
            for x in block.stmts {}
        }
        Ok(NthChild { tree })
    }
}