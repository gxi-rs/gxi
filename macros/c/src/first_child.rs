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
        let mut tree = {
            let name: Ident = input.parse()?;
            let block: Block = input.parse()?;
            quote! { n!(#name init_child #block); }
        };
        //children are optional
        {
            //check for first block
            let content;
            bracketed!(content in input);
            {
                let name: Ident = content.parse()?;
                let block: Block = content.parse()?;
                tree = quote! {#tree n!(#name init_child #block); }
            }
        }
        /*if let Ok(block) = input.parse::<ExprArray>() {
            let block_stream = block.to_token_stream();
            println!("{} ", block_stream.to_string());
            let i = 0;
            for stmt in block.elems {
                println!("stmts {}", stmt.to_token_stream().to_string());
                /*let stmt: TokenStream = stmt.into();
                if i == 0 {
                    tree.append(syn::parse_macro_input!(stmt as FirstChild).tree.into())
                } else {
                    tree.append(syn::parse_macro_input!(stmt as NthChild).tree.into())
                }*/
            }
        } else {
            panic!("not Expr Array");
        }*/
        Ok(FirstChild { tree })
    }
}