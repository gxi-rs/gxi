use crate::component::parse_component_block;
use crate::init_type::InitType;
use quote::TokenStreamExt;
use syn::parse_quote::parse;
use syn::{__private::TokenStream2, parse::Parse};

pub struct TreeParser(pub TokenStream2);

impl Parse for TreeParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Self::custom_parse(&input, InitType::Child)?))
    }
}

impl TreeParser {
    fn custom_parse(
        input: &syn::parse::ParseStream,
        mut init_type: InitType,
    ) -> syn::Result<TokenStream2> {
        if input.is_empty() {
            return Ok(Default::default());
        }

        let mut parsed_buffer = TokenStream2::new();

        loop {
            if let Some(comp) = parse_component_block(input, &init_type)? {
                parsed_buffer.append_all(comp);
            } else {
                // other blocks go here
                println!("hello {}, {}", input.to_string(), input.is_empty());
                todo!()
            }

            if input.is_empty() {
                break;
            } else {
                input.parse::<syn::token::Comma>()?;
            }
        }

        println!("{}", parsed_buffer.to_string());

        Ok(parsed_buffer)
    }
}
