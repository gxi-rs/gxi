use std::collections::HashMap;

use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::Expr;

#[derive(Debug)]
pub enum Scope {
    /// # Args
    ///
    /// observable ident
    Observable(Vec<TokenStream2>),
    Constant,
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Observable(v1), Self::Observable(v2)) => {
                for i in 0..v1.len() {
                    if v1[i].to_string() != v2[i].to_string() {
                        return false;
                    }
                }
                return true;
            }
            (Self::Constant, Self::Constant) => true,
            _ => false,
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::Constant
    }
}

const MORE_THAN_ONE_ERR: &str = "can't have more than one observables in a single expression";

impl Scope {
    pub fn is_const(&self) -> bool {
        if let Scope::Constant = self {
            true
        } else {
            false
        }
    }

    /// find scopes of punctuated expressions
    pub fn find_iter_scope(iter: &mut syn::punctuated::Iter<syn::Expr>) -> syn::Result<Self> {
        let mut k = Scope::default();
        for x in iter {
            let x_scope = Self::find_expr_scope(x)?;
            match (&x_scope, &k) {
                (Scope::Observable(_), Scope::Observable(_)) => {
                    return Err(syn::Error::new(x.span(), MORE_THAN_ONE_ERR));
                }
                (_, Scope::Observable(_)) => {}
                _ => {
                    k = x_scope;
                }
            }
        }
        Ok(k)
    }

    pub fn find_expr_scope(expr: &syn::Expr) -> syn::Result<Self> {
        match expr {
            Expr::Array(syn::ExprArray { elems, .. }) => {
                return Self::find_iter_scope(&mut elems.iter());
            }
            Expr::Binary(syn::ExprBinary { left, right, .. }) => {
                match (Self::find_expr_scope(left)?, Self::find_expr_scope(right)?) {
                    (Scope::Observable(first), Scope::Observable(second)) => {
                        // merge unique observables
                        let mut set = HashMap::with_capacity(first.len().max(second.len()));
                        // insert all values
                        for x in first {
                            set.insert(x.to_string(), x);
                        }
                        for x in second {
                            set.insert(x.to_string(), x);
                        }

                        Ok(Scope::Observable(set.into_values().collect()))
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
            Expr::Cast(syn::ExprCast { expr, .. }) => Scope::find_expr_scope(expr),
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
                    Ok(Self::Observable(vec![segments[0].to_token_stream()]))
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
            Expr::Unary(syn::ExprUnary { expr, .. }) => Self::find_expr_scope(&expr),
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
            | Expr::Yield(_) => Err(syn::Error::new(
                expr.span(),
                "didn't expect this expression here",
            )),
            Expr::Verbatim(_) | Expr::__TestExhaustive(_) => unreachable!(),
        }
    }
}

/// parses input to syn::Expr
impl Parse for Scope {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::find_expr_scope(&input.parse()?)
    }
}

impl Scope {
    // WARN: should be a little less exact
    pub fn to_token_stream(&self, body: &TokenStream2, return_type: &TokenStream2) -> TokenStream2 {
        match &self {
            Scope::Observable(observables) => {
                if observables.len() > 1 {
                    println!("{:?}", observables);
                    unreachable!("more than one observables are not supported yet")
                }
                let name = &observables[0];
                quote! {{
                    let __node = std::rc::Rc::downgrade(&__node);
                    #name.add_observer(Box::new(move |#name| {
                        use std::ops::DerefMut;
                        if let Some(__node) = __node.upgrade() {
                            let mut __node = __node.as_ref().borrow_mut();
                            let __node = __node.deref_mut().as_mut().downcast_mut::<#return_type>().unwrap();

                            #body
                            false
                        } else {
                            true
                        }
                    }));
                }}
            }
            Scope::Constant => quote! {
                #body
            },
        }
    }
}

//TODO: add more tests
#[cfg(test)]
mod expr_init_location {

    use super::Scope;
    use quote::quote;
    use syn::__private::TokenStream2;

    const CONSTANT: bool = true;
    const OBSERVABLE: bool = false;

    fn match_const_scope(expr: TokenStream2, constant: bool) -> syn::Result<bool> {
        if let Scope::Constant = Scope::find_expr_scope(&syn::parse2(expr)?)? {
            Ok(constant)
        } else {
            Ok(!constant)
        }
    }

    #[test]
    fn scope_array() -> syn::Result<()> {
        assert!(match_const_scope(quote! {[1, 2]}, CONSTANT)?);
        assert!(match_const_scope(
            quote! {[1, |_| println!("hello")]},
            CONSTANT
        )?);
        assert!(match_const_scope(
            quote! {[state.a, 3, Hello::hi()]},
            OBSERVABLE
        )?);

        Ok(())
    }

    #[test]
    fn scope_binary_op() -> syn::Result<()> {
        assert!(match_const_scope(quote!(2 == 3), CONSTANT)?);
        assert!(match_const_scope(quote!(2 == "str"), CONSTANT)?);
        assert!(match_const_scope(quote!(2 == state.a), OBSERVABLE)?);
        assert!(match_const_scope(quote!(2 == || {}), CONSTANT)?);
        Ok(())
    }
}
