use quote::{quote, ToTokens, TokenStreamExt};

use crate::{
    component::NodeBlock,
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    scope::Scope,
};

pub struct IfBlock {
    scope: Scope,
    if_token: syn::Token!(if),
    // only 1 node block is allowed in body
    body: Option<NodeBlock>,
    else_arm: Box<Option<ElseArm>>,
    condition: syn::Expr,
}

/// Note: can't exist independently
pub struct ElseArm {
    else_token: syn::Token!(else),
    if_arm: IfBlock,
}

impl OptionalParse for IfBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let if_token = if let Ok(if_token) = input.parse::<syn::Token!(if)>() {
            if_token
        } else {
            return Ok(None);
        };

        // get scope
        let (condition, scope) = {
            let is_const = input.parse::<syn::Token!(const)>().is_ok();
            let condition = input.parse::<syn::Expr>()?;

            // no need to check scope when const keyword is provided
            let scope = if is_const {
                Scope::Constant
            } else {
                Scope::find_expr_scope(&condition)?
            };
            (condition, scope)
        };

        // parse children
        let body = {
            let syn::group::Braces { content, token } = syn::group::parse_braces(&input)?;

            if content.is_empty() {
                None
            } else {
                Some(content.parse::<NodeBlock>()?)
            }
        };

        // else arm
        let else_arm = Box::from(ElseArm::optional_parse(&input)?);

        Ok(Some(Self {
            scope,
            if_token,
            body,
            else_arm,
            condition,
        }))
    }
}

impl_parse_for_optional_parse!(IfBlock);

impl OptionalParse for ElseArm {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        if let Ok(else_token) = input.parse::<syn::Token!(else)>() {
            Ok(Some(Self {
                else_token,
                if_arm: input.parse()?,
            }))
        } else {
            Ok(None)
        }
    }
}

impl_parse_for_optional_parse!(ElseArm);

impl ToTokens for IfBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self {
            scope,
            if_token,
            body,
            else_arm,
            condition,
        } = self;

        tokens.append_all(quote!())
    }
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use crate::scope::Scope;

    use super::IfBlock;

    #[test]
    fn conditional_if_block() -> syn::Result<()> {
        {
            let IfBlock {
                scope,
                condition,
                else_arm,
                body,
                if_token,
            } = syn::parse2(quote! { if t == 3 { div }})?;

            assert_eq!(scope, Scope::Observable(quote! {t}));
            assert_eq!(
                condition.into_token_stream().to_string(),
                quote! { t == 3 }.to_string()
            );
            assert_eq!(body.is_some(), true);
        }
        Ok(())
    }
}
