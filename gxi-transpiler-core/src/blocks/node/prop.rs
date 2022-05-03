use std::ops::{Deref, DerefMut};

use crate::{observer_builder::ObserverBuilder, state::State};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::Token;

/// punctated(,) list of [`NodeProp(s)`](NodeProp) inside parenthesis.
#[derive(Default)]
pub struct NodeProps(pub Vec<NodeProp>);

impl Parse for NodeProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        // parse props
        if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(input) {
            while !content.is_empty() {
                let prop: NodeProp = content.parse()?;
                this.push(prop);
                if !content.is_empty() {
                    content.parse::<syn::token::Comma>()?;
                }
            }
        }

        Ok(this)
    }
}

impl Deref for NodeProps {
    type Target = Vec<NodeProp>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeProps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// # Syntax
/// 
/// ```
/// const? $left = $right
/// ```
///
/// # Example
/// 
/// ```rust
/// name = "gxi-rs"
/// ```
/// 
/// - `$left` = `name`
/// - `$right` = `"gxi-rs"`
///
/// # Scope
/// 
/// - If `const` is present, `scope` = [`Scope::Constant`]
/// otherwise [`scope`](Scope) is set according to `$right`.
///
/// - In `featrue = web`. If `$left` starts with `on` then
///   [`LifeTime`] is set to 
///
pub struct NodeProp {
    pub left: Box<syn::Expr>,
    pub right: Box<syn::Expr>,
    pub scope: State,
    pub requires_context: bool,
}

impl Parse for NodeProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check for * used to mark sccope to be OPen
        let mut scope = State::default();

        #[allow(unused_mut)]
        let mut requires_context = false;

        let const_tt = input.parse::<Token!(const)>();

        let syn::ExprAssign { left, right, .. } = input.parse()?;

        if const_tt.is_err() {
            scope = State::find_expr_scope(&right)?;
        }

        #[cfg(feature = "web")]
        if left.to_token_stream().to_string().starts_with("on") {
            requires_context = true;
        }

        Ok(Self {
            left,
            scope,
            right,
            requires_context,
        })
    }
}

impl ToTokens for NodeProp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            left, right, scope, ..
        } = self;

        tokens.append_all(scope.to_token_stream(&ObserverBuilder {
            pre_add_observer_tokens: &TokenStream2::new(),
            add_observer_body_tokens: &quote! {
                __node.#left(#right);
            },
            borrow: true,
        }))
    }
}
