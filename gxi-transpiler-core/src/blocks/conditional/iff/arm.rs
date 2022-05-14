use quote::{quote, ToTokens};
use syn::{__private::TokenStream2, parse::Parse};

use crate::{
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    state::State,
    sub_tree::SubTree,
};

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
    pub state: State,
}

impl OptionalParse for IfArm {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let if_token = if let Ok(if_token) = input.parse::<syn::Token!(if)>() {
            if_token
        } else {
            return Ok(None);
        };

        // get scope
        let (condition, state) = {
            let condition = input.parse::<syn::Expr>()?;
            // no need to check scope when const keyword is provided
            let state = State::find_expr_scope(&condition)?;
            (condition, state)
        };

        // parse children
        let sub_tree = {
            let syn::group::Braces { content, .. } = syn::group::parse_braces(input)?;

            IfSubTree::parse(&content)?
        };

        Ok(Some(Self {
            if_token,
            condition,
            sub_tree,
            state,
        }))
    }
}

impl_parse_for_optional_parse!(IfArm);

impl IfArm {
    pub fn to_token_stream(&self, arm_index: usize) -> TokenStream2 {
        let Self {
            if_token,
            sub_tree,
            condition,
            state: scope,
        } = self;

        let mut scoped_variables_borrow = TokenStream2::new();
        if let State::Observable(observables) = scope {
            scoped_variables_borrow = observables.borrowed_token_stream();
        }

        quote! {
            #if_token { #scoped_variables_borrow #condition } {
                if __ctx.check_index(#arm_index) {
                    #sub_tree
                }
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

impl Parse for ElseArm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(else_token) = input.parse::<syn::Token!(else)>() {
            if let Some(if_arm) = IfArm::optional_parse(&input)? {
                Ok(Self::WithIfArm {
                    else_token,
                    if_arm: Box::from(if_arm),
                })
            } else {
                let syn::group::Braces { content, .. } = syn::group::parse_braces(input)?;
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
    pub fn to_token_stream(&self, arm_index: usize, constant_scope: bool) -> TokenStream2 {
        match self {
            ElseArm::WithIfArm { else_token, if_arm } => {
                let if_tokens = if_arm.to_token_stream(arm_index);
                quote! { #else_token #if_tokens }
            }
            ElseArm::PureArm { else_token, body } => {
                let body = {
                    let mut tokens = TokenStream2::new();
                    //WARN: arm index breaking change
                    body.to_tokens(&mut tokens);
                    tokens
                };
                quote! {
                    #else_token #body
                }
            }
            ElseArm::None => {
                if constant_scope {
                    TokenStream2::new()
                } else {
                    quote! {
                        //FIX: 
                        else {
                            panic!("pure else arm not implemented yet")
                        }
                    }
                }
            }
        }
    }
}
