use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

enum InitType {
    Child,
    Sibling,
}

pub struct PParser {
    pub tree: TokenStream2
}

impl PParser {
    fn custom_parse(input: ParseStream, init_type: InitType) -> Result<Self> {
        //not mandatory to have a bracket or component inside the macro. macro can be empty
        if let Ok(name) = input.parse::<Ident>() {
            let mut tree = {
                let block = if let Ok(block) = input.parse::<Block>() { block.to_token_stream()} else { (quote!{{}}).into() };
                match init_type {
                    InitType::Child => {
                        quote! { n!(#name init_child #block); }
                    }
                    InitType::Sibling => {
                        quote! { n!(#name init_sibling #block); }
                    }
                }
            };
            {
                //check for first block
                match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = PParser::parse(&brackets.content).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree {  let cont = node.clone(); #content_tree } };
                    }
                    syn::__private::Err(_error) => {}
                }
                //parse ,
                match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = PParser::custom_parse(&input, InitType::Sibling).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree #content_tree };
                    }
                    _ => {}
                }
            }
            return Ok(PParser { tree });
        }
        Ok(PParser { tree: TokenStream2::new() })
    }
}

impl Parse for PParser {
    fn parse(input: ParseStream) -> Result<Self> {
        PParser::custom_parse(input, InitType::Child)
    }
}