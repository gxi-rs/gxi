use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use crate::first_child::FirstChild;

pub struct NthChild {
    pub tree: TokenStream2
}

impl Parse for NthChild {
    fn parse(input: ParseStream) -> Result<Self> {
        //not mandatory to have a bracket or component inside the macro. macro can be empty
        if let Ok(name) = input.parse::<Ident>() {
            let mut tree: TokenStream2 = if let Ok(block) = input.parse::<Block>() {
                quote! { n!(#name init_sibling #block); }
            } else {
                quote! { n!(#name init_sibling {}); }
            };
            {
                //check for first block
                match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = FirstChild::parse(&brackets.content).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree {  let cont = node.clone(); #content_tree } };
                    }
                    syn::__private::Err(_) => {}
                }
                //parse ,
                match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = NthChild::parse(&input).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree #content_tree};
                    }
                    _ => {}
                }
            }
            return Ok(NthChild { tree });
        }
        Ok(NthChild { tree: TokenStream2::new() })
    }
}