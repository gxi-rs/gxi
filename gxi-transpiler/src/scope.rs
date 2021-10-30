use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use syn::Expr;

pub enum Scope {
    Observable(TokenStream2),
    Constant,
}

impl Default for Scope {
    fn default() -> Self {
        Self::Constant
    }
}

const MORE_THAN_ONE_ERR: &str = "can't have more than one observables in a single expression";

impl Scope {
    pub fn find_iter_scope(iter: &mut syn::punctuated::Iter<syn::Expr>) -> syn::Result<Self> {
        let mut k = Scope::default();
        for x in iter {
            let x_scope = Self::find_prop_scope(x)?;
            match (&x_scope, &k) {
                (Scope::Observable(_), Scope::Observable(_)) => {
                    return Err(syn::Error::new(x.span(), MORE_THAN_ONE_ERR));
                }
                _ => {
                    k = x_scope;
                }
            }
        }
        Ok(k)
    }
    pub fn find_prop_scope(expr: &syn::Expr) -> syn::Result<Self> {
        match expr {
            Expr::Array(syn::ExprArray { elems, .. }) => {
                return Self::find_iter_scope(&mut elems.iter());
            }
            Expr::Binary(syn::ExprBinary { left, right, .. }) => {
                match (Self::find_prop_scope(left)?, Self::find_prop_scope(right)?) {
                    (Scope::Observable(_), Scope::Observable(_)) => {
                        Err(syn::Error::new(right.span(), MORE_THAN_ONE_ERR))
                    }
                    (Scope::Observable(name), Scope::Constant) => Ok(Scope::Observable(name)),
                    (Scope::Constant, Scope::Observable(name)) => Ok(Scope::Observable(name)),
                    (Scope::Constant, Scope::Constant) => Ok(Scope::Constant),
                }
            }
            Expr::Block(_) | Expr::Macro(_) | Expr::Lit(_) | Expr::Closure(_) => {
                Ok(Scope::Constant)
            }
            Expr::Call(syn::ExprCall { args, .. }) => Scope::find_iter_scope(&mut args.iter()),
            Expr::Cast(syn::ExprCast { expr, .. }) => Scope::find_prop_scope(expr),
            Expr::Field(syn::ExprField { base, .. }) => Self::find_prop_scope(base),
            Expr::Index(syn::ExprIndex { expr, .. }) => Self::find_prop_scope(expr),
            Expr::ForLoop(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::MethodCall(syn::ExprMethodCall { receiver, .. }) => {
                Self::find_prop_scope(receiver)
            }
            Expr::Paren(syn::ExprParen { expr, .. }) => Self::find_prop_scope(expr),
            Expr::Path(syn::ExprPath { path, .. }) => {
                let segments = &path.segments;
                // path of length 1 is an identifier
                if segments.len() == 1 {
                    Ok(Self::Observable(segments[0].to_token_stream()))
                } else {
                    Ok(Self::Constant)
                }
            }
            Expr::Range(_) => todo!(),
            Expr::Reference(syn::ExprReference { expr, .. }) => Self::find_prop_scope(expr),
            Expr::Repeat(_) => todo!(),
            Expr::Try(_) => todo!(),
            Expr::TryBlock(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Unsafe(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::Assign(_)
            | Expr::Async(_)
            | Expr::Await(_)
            | Expr::Box(_)
            | Expr::Continue(_)
            | Expr::Group(_)
            | Expr::Let(_)
            | Expr::Struct(_)
            | Expr::Type(_)
            | Expr::Break(_)
            | Expr::Return(_)
            | Expr::AssignOp(_)
            | Expr::Yield(_) => Err(syn::Error::new(expr.span(), "didn't expect this here")),
            Expr::Verbatim(_) | Expr::__TestExhaustive(_) => unreachable!(),
        }
    }
}

#[cfg(test)]
mod expr_init_location {

    //    use crate::component::Scope;
    //    use quote::quote;
    //
    //    macro_rules! mp_match {
    //        ($expect:ident, $variant:ident, $($expr:tt)* ) => {
    //        assert_eq!(
    //            Scope::$expect,
    //            Scope::find_prop_scope(
    //                &syn::Expr::$variant(syn::parse2(quote! {
    //                    $($expr)*
    //                })?)
    //            )?
    //        );
    //        };
    //    }

    #[test]
    fn array() -> syn::Result<()> {
        //mp_match!(Constant, Array, [1, 2]);
        //mp_match!(PartialOpen, Array, [1, |_| println!("hello")]);
        //mp_match!(Open, Array, [state.a, 3, Hello::hi()]);
        Ok(())
    }

    #[test]
    fn binary_op() -> syn::Result<()> {
        //mp_match!(Constant, Binary, 2 == 3);
        //mp_match!(Constant, Binary, 2 == "str");
        //mp_match!(Open, Binary, 2 == state.a);
        //mp_match!(PartialOpen, Binary, 2 == || {});
        Ok(())
    }
}
