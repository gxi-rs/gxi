use quote::ToTokens;
use syn::Token;

pub struct ConditionalBlock {}

impl ConditionalBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        if input.peek(Token!(if)) {
            let syn::ExprIf {
                if_token,
                cond,
                then_branch,
                else_branch,
                ..
            } = input.parse::<syn::ExprIf>()?;
        } else if input.peek(Token!(match)) {
            let syn::ExprMatch {
                match_token,
                expr,
                brace_token,
                arms,
                ..
            } = input.parse::<syn::ExprMatch>()?;
        } else {
            return Ok(None);
        }

        Ok(Some(Self {}))
    }
}

impl ToTokens for ConditionalBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        todo!()
    }
}
