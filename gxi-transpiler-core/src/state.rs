use std::ops::AddAssign;

use quote::ToTokens;

use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Expr;

use crate::observables::Observables;

/// State of a expression
#[derive(Debug, Clone)]
pub enum State {
    Observable(Observables),
    Constant,
}

impl PartialEq for State {
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl AddAssign for State {
    fn add_assign(&mut self, rhs: Self) {
        match (self, rhs) {
            (State::Observable(s), State::Observable(ref mut o)) => s.append(o),
            (this @ State::Constant, rhs @ State::Observable(_)) => *this = rhs,
            _ => (),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Constant
    }
}

impl State {
    pub fn is_const(&self) -> bool {
        matches!(self, State::Constant)
    }

    /// find scopes of punctuated expressions
    pub fn find_iter_scope(iter: &mut syn::punctuated::Iter<syn::Expr>) -> syn::Result<Self> {
        let mut observables = Observables::default();

        for x in iter {
            if let State::Observable(mut expr_observables) = Self::find_expr_scope(x)? {
                observables.append(&mut expr_observables);
            };
        }

        Ok(if observables.is_empty() {
            State::Constant
        } else {
            observables.remove_duplicates();
            State::Observable(observables)
        })
    }

    /// FIX: take token steam or vec of Expr
    pub fn find_expr_scope(expr: &syn::Expr) -> syn::Result<Self> {
        match expr {
            Expr::Array(syn::ExprArray { elems, .. }) => {
                return Self::find_iter_scope(&mut elems.iter());
            }
            Expr::Binary(syn::ExprBinary { left, right, .. }) => {
                // binary expressions may have repeated variable names
                // filter them
                match (Self::find_expr_scope(left)?, Self::find_expr_scope(right)?) {
                    (State::Observable(mut first), State::Observable(mut second)) => {
                        // remove duplicaes
                        first.append(&mut second);
                        first.remove_duplicates();
                        Ok(State::Observable(first))
                    }
                    (State::Observable(name), State::Constant) => Ok(State::Observable(name)),
                    (State::Constant, State::Observable(name)) => Ok(State::Observable(name)),
                    (State::Constant, State::Constant) => Ok(State::Constant),
                }
            }
            Expr::Block(_) | Expr::Macro(_) | Expr::Lit(_) | Expr::Closure(_) => {
                Ok(State::Constant)
            }
            Expr::Call(syn::ExprCall { args, .. }) => State::find_iter_scope(&mut args.iter()),
            Expr::Cast(syn::ExprCast { expr, .. }) => State::find_expr_scope(expr),
            Expr::Field(syn::ExprField { base, .. }) => Self::find_expr_scope(base),
            Expr::Index(syn::ExprIndex { expr, .. }) => Self::find_expr_scope(expr),
            Expr::ForLoop(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::MethodCall(syn::ExprMethodCall { receiver, .. }) => {
                Self::find_expr_scope(receiver)
            }
            Expr::Paren(syn::ExprParen { expr, .. }) => Self::find_expr_scope(expr),
            Expr::Path(syn::ExprPath { path, .. }) => {
                let segments = &path.segments;
                // path of length 1 is an identifier
                if segments.len() == 1 {
                    Ok(Self::Observable(Observables(vec![
                        segments[0].to_token_stream()
                    ])))
                } else {
                    Ok(Self::Constant)
                }
            }
            Expr::Range(_) => todo!(),
            Expr::Reference(syn::ExprReference { expr, .. }) => Self::find_expr_scope(expr),
            Expr::Repeat(_) => todo!(),
            Expr::Try(_) => todo!(),
            Expr::TryBlock(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::Unary(syn::ExprUnary { expr, .. }) => Self::find_expr_scope(expr),
            Expr::Unsafe(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::Async(_)
            | Expr::Await(_)
            | Expr::Continue(_)
            | Expr::Let(_)
            | Expr::Struct(_)
            | Expr::Break(_)
            | Expr::Return(_)
            | Expr::Assign(_)
            | Expr::Group(_)
            | Expr::Yield(_) => Err(syn::Error::new(
                expr.span(),
                format!(
                    "[gxi] Unexpected expression {} Refer to https://github.com/gxi-rs/gxi/issues/31",
                    expr.to_token_stream()
                ),
            )),
            _ => unreachable!(),
        }
    }
}

/// parses input to syn::Expr
impl Parse for State {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::find_expr_scope(&input.parse()?)
    }
}

pub trait StateExt {
    /// state of self
    fn get_state(&self) -> State;

    /// state of the nested tree
    fn get_nested_state(&self) -> State;
}

//TODO: add more tests
//FIX: add any number of expressions
#[cfg(test)]
mod expr_init_location {

    use super::State;
    use anyhow::ensure;
    use quote::quote;
    use syn::__private::TokenStream2;

    const CONSTANT: bool = true;
    const OBSERVABLE: bool = false;

    fn match_const_scope(expr: TokenStream2, constant: bool) -> anyhow::Result<bool> {
        if let State::Constant = State::find_expr_scope(&syn::parse2(expr)?)? {
            Ok(constant)
        } else {
            Ok(!constant)
        }
    }

    #[test]
    fn scope_array() -> anyhow::Result<()> {
        ensure!(match_const_scope(quote! {[1, 2]}, CONSTANT)?);
        ensure!(match_const_scope(
            quote! {[1, |_| println!("hello")]},
            CONSTANT
        )?);
        ensure!(match_const_scope(
            quote! {[state.a, 3, Hello::hi()]},
            OBSERVABLE
        )?);

        Ok(())
    }

    #[test]
    fn scope_binary_op() -> anyhow::Result<()> {
        ensure!(match_const_scope(quote!(2 == 3), CONSTANT)?);
        ensure!(match_const_scope(quote!(2 == "str"), CONSTANT)?);
        ensure!(match_const_scope(quote!(2 == state.a), OBSERVABLE)?);
        ensure!(match_const_scope(quote!(2 == || {}), CONSTANT)?);
        Ok(())
    }
}
