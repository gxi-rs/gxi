use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Expr;

pub struct ComponentProp {
    pub left: Box<syn::Expr>,
    pub right: Box<syn::Expr>,
    pub scope: Scope,
}

impl Parse for ComponentProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let syn::ExprAssign {
            left, mut right, ..
        } = input.parse()?;
        let scope = Scope::find_prop_scope(&mut right)?;
        Ok(Self { left, scope, right })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Scope {
    /// the value wont change i.e constant
    Constant,
    /// the value is constant but needs to be kept in a partial open env.
    /// may or may not depend on the env
    /// eg in a if statement which executes once
    /// eg closures, need to assigned once
    PartialOpen,
    /// the value is dependent on external factors and may change
    /// eg. state
    Open,
}

impl Default for Scope {
    fn default() -> Self {
        Self::Constant
    }
}

impl Scope {
    pub fn is_serializable(&self) -> bool {
        match self {
            Self::Constant => true,
            _ => false,
        }
    }
    /// compare and promote self if others scope is higher
    pub fn comp_and_promote(&mut self, other: &Self) {
        let scope = match (&self, other) {
            (Self::Constant, _) => other.clone(),
            (Self::PartialOpen, Self::Constant | Self::PartialOpen) => Self::PartialOpen,
            (Self::PartialOpen, Self::Open) => Self::Open,
            (Self::Open, _) => Self::Open,
        };
        *self = scope;
    }

    fn find_prop_scope(expr: &mut syn::Expr) -> syn::Result<Self> {
        return match expr {
            Expr::Lit(_) => Ok(Self::Constant),

            // find a way to edit expr
            Expr::Closure(syn::ExprClosure {
                body,
                asyncness,
                output,
                ..
            }) => {
                if asyncness.is_some() {
                    return Err(syn::Error::new(
                        expr.span(),
                        "async closures are not supported yet. Use async update function instead.",
                    ));
                }

                if !output.to_token_stream().is_empty() {
                    return Err(syn::Error::new(
                        expr.span(),
                        "this closure cannot return a value",
                    ));
                }

                // is_new_props.append_all(quote! {
                //     __node.#left(
                //         #(#attrs)*
                //         move |#inputs| {
                //             #body
                //         }
                //     );
                // }) * expr = Expr::Closure(syn::parse2::<syn::ExprClosure>(quote! {
                //     ||
                // })?);
                Ok(Self::PartialOpen)
            }

            Expr::Array(syn::ExprArray { elems, .. }) => {
                for x in elems {
                    let k = Self::find_prop_scope(x)?;

                    match k {
                        Self::Constant => continue,
                        _ => return Ok(k),
                    }
                }
                Ok(Self::Constant)
            }
            Expr::AssignOp(_) => todo!(),
            Expr::Binary(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::ForLoop(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Index(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::Macro(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::MethodCall(_) => todo!(),
            Expr::Paren(_) => todo!(),
            Expr::Path(_) => todo!(),
            Expr::Range(_) => todo!(),
            Expr::Reference(_) => todo!(),
            Expr::Repeat(_) => todo!(),
            Expr::Try(_) => todo!(),
            Expr::TryBlock(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Unsafe(_) => todo!(),
            Expr::Assign(_)
            | Expr::Async(_)
            | Expr::Await(_)
            | Expr::Box(_)
            | Expr::Continue(_)
            | Expr::Group(_)
            | Expr::Let(_)
            | Expr::Struct(_)
            | Expr::Field(_)
            | Expr::Type(_)
            | Expr::Break(_)
            | Expr::Return(_)
            | Expr::Yield(_) => Err(syn::Error::new(expr.span(), "didn't expect this here")),
            Expr::Verbatim(_) | Expr::__TestExhaustive(_) => unreachable!(),
        };
    }
}

#[cfg(test)]
mod expr_init_location {

    use crate::component::ExprInitLocation;
    use quote::quote;

    struct MyParser(syn::Expr);
    impl syn::parse::Parse for MyParser {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(input.parse::<syn::Expr>()?))
        }
    }

    macro_rules! mp_match {
        ($expect:ident, $($expr:tt)* ) => {
        assert_eq!(
            ExprInitLocation::$expect,
            ExprInitLocation::find(
                &syn::parse2::<MyParser>(quote! {
                    $($expr)*
                })?
                .0,
            )?
        );
        };
    }

    #[test]
    fn array() -> syn::Result<()> {
        mp_match!(Constant, [1, 2]);
        mp_match!(Once, [1, || { println!!("hello") }]);
        mp_match!(Open, [state.2, 3, Hello::hi()]);
        Ok(())
    }
}
