use quote::{quote, ToTokens, TokenStreamExt};

use crate::optional_parse::{impl_parse_for_optional_parse, OptionalParse};

pub struct ExecutionBlock(syn::Block);

impl OptionalParse for ExecutionBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        if let Ok(block) = input.parse::<syn::Block>() {
            return Ok(Some(Self(block)));
        }
        Ok(None)
    }
}

impl_parse_for_optional_parse!(ExecutionBlock);

impl ToTokens for ExecutionBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let stmts = &self.0.stmts;
        tokens.append_all(quote! {
            #(#stmts)*
        })
    }
}
