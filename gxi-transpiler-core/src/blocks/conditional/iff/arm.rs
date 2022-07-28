use std::ops::Range;

use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse::Parse;

use crate::{observables::Observables, state::State, sub_tree::SubTree};

use super::subtree::IfSubTree;

/// ## Syntax
///
/// ```rust
/// if $condition {
///     $subtree?
/// }
/// ```
pub struct IfArm {
    pub if_token: syn::Token!(if),
    pub condition: syn::Expr,
    pub sub_tree: IfSubTree,
    /// splice range of state from IfBlock
    pub observables_state_range: Range<usize>,
}

impl IfArm {
    pub fn optional_parse(
        input: &syn::parse::ParseStream,
        if_block_observable_state: &mut Observables,
    ) -> syn::Result<Option<Self>> {
        let if_token = if let Ok(if_token) = input.parse::<syn::Token!(if)>() {
            if_token
        } else {
            return Ok(None);
        };

        // get scope
        let (condition, observables_state_range) = {
            let condition = input.parse::<syn::Expr>()?;

            let splice_start_index = if_block_observable_state.len();

            if let State::Observable(Observables(observables)) = State::find_expr_scope(&condition)?
            {
                if_block_observable_state.extend(observables.into_iter());
            };

            (
                condition,
                splice_start_index..if_block_observable_state.len(),
            )
        };

        // parse children
        let sub_tree = {
            let bracket = syn::__private::parse_braces(input)?;
            IfSubTree::parse(&bracket.content)?
        };

        Ok(Some(Self {
            if_token,
            condition,
            sub_tree,
            observables_state_range,
        }))
    }
}

impl Parse for IfArm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self::optional_parse(&input, &mut Observables::default())?.unwrap())
    }
}

impl IfArm {
    pub fn to_token_stream(&self, arm_index: usize, state: &State) -> TokenStream2 {
        let Self {
            if_token,
            sub_tree: _,
            condition,
            observables_state_range,
        } = self;

        let conditon = if let State::Observable(observables) = state {
            let borrow = observables.borrowed_token_stream(observables_state_range);
            quote!({
                #borrow
                #condition
            })
        } else {
            condition.to_token_stream()
        };

        let mut sub_tree = TokenStream2::new();
        self.sub_tree.to_tokens(&mut sub_tree, state);

        let sub_tree = if let State::Constant = state {
            quote!(
                #sub_tree
            )
        } else {
            quote!(
                if __ctx.check_index(#arm_index) {
                    #sub_tree
                }
            )
        };

        quote! {
            #if_token #conditon {
                #sub_tree
            }
        }
    }
}

// NOTE: can't exist independently
pub enum ElseArm {
    WithIfArm {
        else_token: syn::Token!(else),
        if_arm: Box<IfArm>,
    },
    PureArm {
        else_token: syn::Token!(else),
        body: IfSubTree,
    },
    None,
}

impl ElseArm {
    pub fn parse(
        input: syn::parse::ParseStream,
        if_block_observable_state: &mut Observables,
    ) -> syn::Result<Self> {
        if let Ok(else_token) = input.parse::<syn::Token!(else)>() {
            if let Some(if_arm) = IfArm::optional_parse(&input, if_block_observable_state)? {
                Ok(Self::WithIfArm {
                    else_token,
                    if_arm: Box::from(if_arm),
                })
            } else {
                let content = syn::__private::parse_braces(input)?.content;

                Ok(Self::PureArm {
                    else_token,
                    body: IfSubTree::parse(&content)?,
                })
            }
        } else {
            Ok(Self::None)
        }
    }
}

impl ElseArm {
    /// constant_scope: true if the IFBlock's state `is_const()`
    pub fn to_token_stream(&self, arm_index: usize, state: &State) -> TokenStream2 {
        match self {
            ElseArm::WithIfArm { else_token, if_arm } => {
                let if_tokens = if_arm.to_token_stream(arm_index, state);
                quote! { #else_token #if_tokens }
            }
            ElseArm::PureArm { else_token, body } => {
                let body = {
                    let mut tokens = TokenStream2::new();
                    //WARN: arm index breaking change
                    body.to_tokens(&mut tokens, state);
                    tokens
                };
                quote! {
                    #else_token #body
                }
            }
            ElseArm::None => {
                if let State::Constant = state {
                    TokenStream2::new()
                } else {
                    quote! {
                        //FIX:
                        else {
                            panic!("if block without else arm not yet implemented")
                        }
                    }
                }
            }
        }
    }
}
