use quote::{quote, TokenStreamExt};
use syn::{__private::TokenStream2, parse::Parse};

use crate::{
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    scope::Scope,
};

use super::subtree::{IfSubBlock, IfSubTree};

pub struct IfArm {
    pub scope: Scope,
    pub if_token: syn::Token!(if),
    pub sub_tree: IfSubTree,
    pub else_arm: Box<ElseArm>,
    pub condition: syn::Expr,
}

impl OptionalParse for IfArm {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let if_token = if let Ok(if_token) = input.parse::<syn::Token!(if)>() {
            if_token
        } else {
            return Ok(None);
        };

        // get scope
        let (condition, scope) = {
            let condition = input.parse::<syn::Expr>()?;
            // no need to check scope when const keyword is provided
            let scope = Scope::find_expr_scope(&condition)?;
            (condition, scope)
        };

        // parse children
        let body = {
            let syn::group::Braces { content, .. } = syn::group::parse_braces(input)?;

            content.parse()?
        };

        // else arm
        let else_arm = input.parse::<ElseArm>()?.into();

        Ok(Some(Self {
            scope,
            if_token,
            sub_tree: body,
            else_arm,
            condition,
        }))
    }
}

impl_parse_for_optional_parse!(IfArm);

impl IfArm {
    pub fn to_token_stream(
        &self,
        branch_index: usize,
        node_blocks: usize,
        max_node_height: usize,
        parent_return_type: &TokenStream2,
        constant_scope: bool,
    ) -> TokenStream2 {
        let Self {
            if_token,
            sub_tree,
            else_arm,
            condition,
            scope,
        } = self;

        let else_arm = else_arm.to_token_stream(
            branch_index + 1,
            node_blocks,
            max_node_height,
            parent_return_type,
            constant_scope,
        );

        let sub_tree = {
            let mut tokens = TokenStream2::new();
            sub_tree.to_tokens(
                &mut tokens,
                branch_index,
                node_blocks,
                max_node_height,
                parent_return_type,
            );
            tokens
        };

        let mut scoped_variables_borrow = TokenStream2::new();
        if let Scope::Observable(observables) = scope {
            for observable in observables {
                scoped_variables_borrow.append_all(quote! {
                    let #observable = (**#observable).borrow();
                });
            }
        }

        quote! {
            #if_token { #scoped_variables_borrow #condition } {
                #sub_tree
            } #else_arm
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
                    body: content.parse()?,
                })
            }
        } else {
            Ok(Self::None)
        }
    }
}

impl ElseArm {
    pub fn to_token_stream(
        &self,
        branch_index: usize,
        node_index: usize,
        max_node_height: usize,
        parent_return_type: &TokenStream2,
        constant_scope: bool,
    ) -> TokenStream2 {
        match self {
            ElseArm::WithIfArm { else_token, if_arm } => {
                let if_tokens = if_arm.to_token_stream(
                    branch_index,
                    node_index,
                    max_node_height,
                    parent_return_type,
                    constant_scope,
                );
                quote! { #else_token #if_tokens }
            }
            ElseArm::PureArm { else_token, body } => {
                let body = {
                    let mut tokens = TokenStream2::new();
                    body.to_tokens(
                        &mut tokens,
                        branch_index,
                        node_index,
                        max_node_height,
                        parent_return_type,
                    );
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
                    let mut body = IfSubTree::default();
                    body.push(IfSubBlock::NoneBlock);
                    let body = {
                        let mut tokens = TokenStream2::new();
                        body.to_tokens(
                            &mut tokens,
                            branch_index,
                            node_index,
                            max_node_height,
                            parent_return_type,
                        );
                        tokens
                    };
                    quote! {
                        else {
                            #body
                        }
                    }
                }
            }
        }
    }
}
