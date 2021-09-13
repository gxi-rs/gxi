use quote::{ToTokens, TokenStreamExt, quote};

pub struct ExecutionBlock(syn::Block);

impl ExecutionBlock {
    pub fn parse(input: syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        if let Ok(block) = input.parse::<syn::Block>() {
            return Ok(Some(Self(block)));
        }
        Ok(None)
    }
}

impl ToTokens for ExecutionBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let stmts = &self.0.stmts;
        tokens.append_all(quote! {
            #(#stmts)*
        })
    }
}
