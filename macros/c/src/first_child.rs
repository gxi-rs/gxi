use proc_macro::{Punct, TokenStream};

use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::group::Brackets;
use syn::parse::{Parse, ParseStream};
use syn::token::Token;

use crate::nth_child::NthChild;

pub struct FirstChild {
    pub tree: TokenStream2
}

impl Parse for FirstChild {
    fn parse(input: ParseStream) -> Result<Self> {
        //not mandatory to have a bracket or component inside the macro. macro can be empty
        if let Ok(name) = input.parse::<Ident>() {
            let mut tree = if let Ok(block) = input.parse::<Block>() {
                quote! { n!(#name init_child #block); }
            } else {
                quote! { n!(#name init_child {}); }
            };
            {
                //check for first block
                match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = brackets.content;
                        let name: Ident = content.parse()?;
                        let block: Block = content.parse()?;
                        tree = quote! {#tree n!(#name init_child #block); }
                    }
                    syn::__private::Err(error) => {}
                }
                //parse ,
            }
            return Ok(FirstChild { tree });
        }
        Ok(FirstChild { tree: TokenStream2::new() })
    }
}