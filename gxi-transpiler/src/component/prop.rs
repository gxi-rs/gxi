use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::Expr;

/// list of comma separated props inside parenthesis
#[derive(Default)]
pub struct NodeProps {
    pub props: Vec<NodeProp>,
    pub scope: Scope,
}

impl Parse for NodeProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        // parse props
        if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(&input) {
            loop {
                if content.is_empty() {
                    break;
                }
                let prop: NodeProp = content.parse()?;
                this.scope.comp_and_promote(&prop.scope);
                this.props.push(prop);
                if !content.is_empty() {
                    content.parse::<syn::token::Comma>()?;
                } else {
                    break;
                }
            }
        }

        Ok(this)
    }
}

pub struct NodeProp {
    pub left: Box<syn::Expr>,
    pub right: Box<syn::Expr>,
    pub scope: Scope,
}

impl Parse for NodeProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check for * used to mark sccope to be OPen
        // TODO: update docs about *
        // TODO: add to docs, if not explicitly marked, each prop has a scope of PartialOpen if it
        // TODO: is not a constant or a filed( eg state.a )
        
        let mut scope = Scope::default();

        if let Ok(_) = input.parse::<syn::Token!(*)>() {
            scope = Scope::Open;
        }

        let syn::ExprAssign { left, right, .. } = input.parse()?;

        //TODO: add event listener handler using *= token

        if let Scope::Constant = scope {
            scope = Scope::find_prop_scope(&right)?;
        }

        Ok(Self { left, scope, right })
    }
}

impl ToTokens for NodeProp {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let left = &self.left;
        let right = &self.right;

        tokens.append_all(quote! {
            __node.#left(#right);
        })
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

    fn find_prop_scope(expr: &syn::Expr) -> syn::Result<Self> {
        //TODO: complete this
        return match expr {
            Expr::Lit(_) => Ok(Self::Constant),
            Expr::Field(_) => Ok(Self::Open),
            Expr::Closure(_) => Ok(Self::PartialOpen),
            Expr::Array(syn::ExprArray { elems, .. }) => {
                let mut scope = Scope::Constant;
                for x in elems {
                    let k = Self::find_prop_scope(x)?;
                    scope.comp_and_promote(&k);
                    if let Scope::Open = k {
                        break;
                    }
                }
                Ok(scope)
            }
            Expr::Binary(syn::ExprBinary { left, right, .. }) => {
                let mut scope = Scope::default();
                scope.comp_and_promote(&Self::find_prop_scope(&left)?);
                scope.comp_and_promote(&Self::find_prop_scope(&right)?);
                Ok(scope)
            }
            //TODO: update docs, blocks wont update with state
            Expr::Block(_) => Ok(Scope::PartialOpen),
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
            | Expr::Type(_)
            | Expr::Break(_)
            | Expr::Return(_)
            | Expr::AssignOp(_)
            | Expr::Yield(_) => Err(syn::Error::new(expr.span(), "didn't expect this here")),
            Expr::Verbatim(_) | Expr::__TestExhaustive(_) => unreachable!(),
        };
    }
}

#[cfg(test)]
mod expr_init_location {

    use crate::Scope;
    use quote::quote;

    macro_rules! mp_match {
        ($expect:ident, $variant:ident, $($expr:tt)* ) => {
        assert_eq!(
            Scope::$expect,
            Scope::find_prop_scope(
                &syn::Expr::$variant(syn::parse2(quote! {
                    $($expr)*
                })?)
            )?
        );
        };
    }

    #[test]
    fn array() -> syn::Result<()> {
        mp_match!(Constant, Array, [1, 2]);
        mp_match!(PartialOpen, Array, [1, |_| println!("hello")]);
        mp_match!(Open, Array, [state.a, 3, Hello::hi()]);
        Ok(())
    }

    #[test]
    fn binary_op() -> syn::Result<()> {
        mp_match!(Constant, Binary, 2 == 3);
        mp_match!(Constant, Binary, 2 == "str");
        mp_match!(Open, Binary, 2 == state.a);
        mp_match!(PartialOpen, Binary, 2 == || {});
        Ok(())
    }
}
