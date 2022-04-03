use quote::ToTokens;

use crate::optional_parse::{impl_parse_for_optional_parse, OptionalParse};

pub struct MatchBlock {
    //    scope: Scope,
//    expr: Expr,
}

impl OptionalParse for MatchBlock {
    fn optional_parse(_input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        todo!()
        //        let mut scope = if let Ok(_) = input.parse::<Token!(const)>() {
        //            Some(Scope::Constant)
        //        } else {
        //            None
        //        };
        //
        //        let expr = if input.peek(Token!(if)) {
        //            let if_expr = input.parse::<syn::ExprIf>()?;
        //
        //            if let None = scope {
        //                scope = Some(Scope::find_expr_scope(&if_expr.cond)?);
        //            }
        //
        //            Expr::If(if_expr)
        //        } else if input.peek(Token!(match)) {
        //            let match_expr = input.parse::<syn::ExprMatch>()?;
        //
        //            if let None = scope {
        //                scope = Some(Scope::find_expr_scope(&match_expr.expr)?);
        //            }
        //
        //            Expr::Match(match_expr)
        //        } else {
        //            return Ok(None);
        //        };
        //
        //        Ok(Some(Self {
        //            scope: scope.unwrap(),
        //            expr,
        //        }))
    }
}

impl_parse_for_optional_parse!(MatchBlock);

impl ToTokens for MatchBlock {
    fn to_tokens(&self, _tokens: &mut quote::__private::TokenStream) {
        todo!()
        //let Self { scope, expr } = self;
        //match expr {
        //    Expr::Match(match_expr) => {}
        //    Expr::If(if_expr) => {}
        //    _ => unreachable!("Internal Logic Error"),
        //}
    }
}

#[cfg(test)]
mod tests {}
