use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

enum InitType {
    Child,
    Sibling,
}

pub struct CParser {
    pub tree: TokenStream2
}

impl CParser {
    fn custom_parse(input: ParseStream, init_type: InitType) -> Result<Self> {
        //not mandatory to have a bracket or component inside the macro. macro can be empty
        if let Ok(name) = input.parse::<Ident>() {
            let mut tree: TokenStream2 = if let Ok(block) = input.parse::<Block>() {
                match init_type {
                    InitType::Child => {
                        quote! { n!(#name init_child #block); }
                    }
                    InitType::Sibling => {
                        quote! { n!(#name init_sibling #block); }
                    }
                }
            } else {
                match init_type {
                    InitType::Child => {
                        quote! { n!(#name init_child {}); }
                    }
                    InitType::Sibling => {
                        quote! { n!(#name init_sibling {}); }
                    }
                }
            };
            {
                //check for first block
                match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = CParser::parse(&brackets.content).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree {  let cont = node.clone(); #content_tree } };
                    }
                    syn::__private::Err(_error) => {}
                }
                //parse ,
                match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = CParser::custom_parse(&input, InitType::Sibling).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree #content_tree };
                    }
                    _ => {}
                }
            }
            return Ok(CParser { tree });
        }
        Ok(CParser { tree: TokenStream2::new() })
    }
}

impl Parse for CParser {
    fn parse(input: ParseStream) -> Result<Self> {
        CParser::custom_parse(input, InitType::Child)
    }
}