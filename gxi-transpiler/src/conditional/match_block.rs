use quote::ToTokens;
use syn::{parse::Parse, Expr, Token};

use crate::{
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    scope::Scope,
};

pub struct MatchBlock {
    scope: Scope,
    expr: Expr,
}

impl OptionalParse for MatchBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let mut scope = if let Ok(_) = input.parse::<Token!(const)>() {
            Some(Scope::Constant)
        } else {
            None
        };

        let expr = if input.peek(Token!(if)) {
            let if_expr = input.parse::<syn::ExprIf>()?;

            if let None = scope {
                scope = Some(Scope::find_expr_scope(&if_expr.cond)?);
            }

            Expr::If(if_expr)
        } else if input.peek(Token!(match)) {
            let match_expr = input.parse::<syn::ExprMatch>()?;

            if let None = scope {
                scope = Some(Scope::find_expr_scope(&match_expr.expr)?);
            }

            Expr::Match(match_expr)
        } else {
            return Ok(None);
        };

        println!("asdf");

        Ok(Some(Self {
            scope: scope.unwrap(),
            expr,
        }))
    }
}

impl_parse_for_optional_parse!(MatchBlock);

impl ToTokens for MatchBlock {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self { scope, expr } = self;
        match expr {
            Expr::Match(match_expr) => {}
            Expr::If(if_expr) => {}
            _ => unreachable!("Internal Logic Error"),
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::Expr;

    use crate::{conditional::MatchBlock, scope::Scope};

    #[test]
    fn conditional_block() -> syn::Result<()> {
        //        {
        //            let MatchBlock { expr, scope } =
        //                syn::parse2(quote! { if t == 3 { div [  ] }})?;
        //            assert_eq!(scope, Scope::Constant);
        //            assert_eq!(if let Expr::If(_) = expr { true } else { false }, true);
        //        }
        Ok(())
    }
}
