use quote::{quote, TokenStreamExt};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Token};

/// list of comma separated props inside parenthesis
#[derive(Default)]
pub struct NodeProps {
    pub props: Vec<NodeProp>,
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
        let mut scope = Scope::default();

        // TODO: add to doc, const

        let const_tt = input.parse::<Token!(const)>();

        let syn::ExprAssign { left, right, .. } = input.parse()?;

        if let Err(_) = const_tt {
            scope = Scope::find_prop_scope(&right)?;
        }

        Ok(Self { left, scope, right })
    }
}

impl NodeProp {
    pub fn to_tokens(&self, tokens: &mut quote::__private::TokenStream, path: &syn::Path) {
        let left = &self.left;
        let right = &self.right;

        tokens.append_all(match &self.scope {
            Scope::Observable(name) => quote! {
                let __node = __node.clone();
                #name.add_observer(Box::new(|#name| {
                    use std::ops::DerefMut;

                    let mut __node = __node.as_ref().borrow_mut();
                    let __node = __node.deref_mut().as_mut().downcast_mut::<#path>().unwrap();

                    __node.#left(#right);
                }));
            },
            Scope::Constant => quote! {
                __node.#left(#right);
            },
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Scope {
    Observable(syn::Ident),
    Constant,
}

impl Default for Scope {
    fn default() -> Self {
        Self::Constant
    }
}

impl Scope {
    fn find_prop_scope(expr: &syn::Expr) -> syn::Result<Self> {
        match expr {
            Expr::Array(_) => todo!(),
            Expr::Binary(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::Closure(_) => todo!(),
            Expr::Field(_) => todo!(),
            Expr::ForLoop(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Index(_) => todo!(),
            Expr::Lit(_) => todo!(),
            Expr::Loop(_) => todo!(),
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

    use crate::component::Scope;
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
