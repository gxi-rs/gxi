use std::ops::{Deref, DerefMut};

use crate::lifetime::LifeTime;
use crate::observables::Observables;
use crate::observer_builder::ObserverBuilder;
use crate::state::State;
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
        if let Ok(parens) = syn::__private::parse_parens(input) {
            let content = parens.content;
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

impl From<&NodeProps> for State {
    fn from(props: &NodeProps) -> Self {
        let mut observables = Vec::default();

        for prop in props.iter() {
            if let State::Observable(mut o) = prop.state.clone() {
                observables.append(&mut o)
            }
        }

        if observables.is_empty() {
            State::Constant
        } else {
            State::Observable(Observables(observables))
        }
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
    pub state: State,
    pub lifetime: LifeTime,
}

impl Parse for NodeProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check for * used to mark sccope to be OPen
        let mut scope = State::default();

        let mut lifetime = LifeTime::Constant;

        let const_tt = input.parse::<Token!(const)>();

        let syn::ExprAssign { left, right, .. } = input.parse()?;

        if const_tt.is_err() {
            scope = State::find_expr_scope(&right)?;
        }

        if let State::Observable(_) = scope {
            lifetime = LifeTime::Context(Default::default());
        }

        // event listner needs an extended lifetime
        #[cfg(feature = "web")]
        if left.to_token_stream().to_string().starts_with("on") {
            lifetime = LifeTime::Context(Default::default());
        }

        Ok(Self {
            left,
            state: scope,
            right,
            lifetime,
        })
    }
}

impl ToTokens for NodeProp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            left,
            right,
            state: scope,
            ..
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
