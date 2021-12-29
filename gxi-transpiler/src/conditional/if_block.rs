use std::collections::HashMap;

use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

use crate::{
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    scope::Scope,
};
use else_arm::ElseArm;

use self::if_arm::IfArm;

use super::{ConditionalBlock, IfSubBlock};

pub struct IfBlock {
    pub if_arm: IfArm,
    pub scope: Scope,
    // max possible height of vertical nodes in the nested subtree
    pub max_node_height: usize,
}

impl OptionalParse for IfBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let if_arm = if let Some(if_arm) = IfArm::optional_parse(input)? {
            if_arm
        } else {
            return Ok(None);
        };

        let mut scoped_variables = HashMap::new();
        let mut max_node_height = 0usize;

        {
            let mut if_arm_ = &if_arm;
            loop {
                let mut max_arm_node_height = 0usize;

                if let Scope::Observable(observables) = &if_arm.scope {
                    for x in observables {
                        scoped_variables.insert(x.to_string(), x.clone());
                    }
                }

                // calculate max_pre_allocations
                for block in if_arm.sub_tree.iter() {
                    match block {
                        IfSubBlock::Node(_) => {
                            max_arm_node_height += 1;
                        }
                        IfSubBlock::Conditional(cond) => match &cond {
                            ConditionalBlock::If(if_block) => {
                                max_arm_node_height += if_block.max_node_height;
                            }
                            ConditionalBlock::Match(_) => {
                                todo!("[if_subtree] match expressions not yet implemented")
                            }
                        },
                        IfSubBlock::Execution(_) => (),
                        IfSubBlock::NoneBlock => unreachable!(),
                    }
                }

                max_node_height = max_node_height.max(max_arm_node_height);

                match &*if_arm_.else_arm {
                    ElseArm::WithIfArm { if_arm, .. } => if_arm_ = if_arm,
                    _ => {
                        break;
                    }
                }
            }
        }

        Ok(Some(Self {
            if_arm,
            scope: if scoped_variables.is_empty() {
                Scope::Constant
            } else {
                Scope::Observable(scoped_variables.into_values().collect())
            },
            max_node_height,
        }))
    }
}
// __index_offset = index of first if statement
// formula = Math::min(__index_offset, INDEX);
// __index_offset+=1;

impl IfBlock {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        node_index: usize,
        parent_return_type: &TokenStream2,
    ) {
        let mut if_arm_tokens = self.if_arm.to_token_stream(
            1,
            node_index,
            self.max_node_height,
            parent_return_type,
            self.scope.is_const(),
        );

        if !self.scope.is_const() {
            if_arm_tokens = quote! {
                use std::ops::{DerefMut, Deref};

                #if_arm_tokens
            };
        }

        let mut main_body = self
            .scope
            .to_token_stream(&if_arm_tokens, parent_return_type);

        if let Scope::Observable(_) = self.scope {
            main_body = quote! {
                let mut __if_counter = 0usize;
                #main_body
            }
        }

        tokens.append_all(main_body);
    }
}

impl_parse_for_optional_parse!(IfBlock);

mod if_arm {
    use crate::conditional::{if_block::else_arm::ElseArm, IfSubTree};
    use quote::quote;
    use syn::__private::TokenStream2;

    use crate::{
        optional_parse::{impl_parse_for_optional_parse, OptionalParse},
        scope::Scope,
    };

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
                let is_const = input.parse::<syn::Token!(const)>().is_ok();
                let condition = input.parse::<syn::Expr>()?;
                // no need to check scope when const keyword is provided
                let scope = if is_const {
                    Scope::Constant
                } else {
                    Scope::find_expr_scope(&condition)?
                };
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
            node_index: usize,
            max_node_height: usize,
            parent_return_type: &TokenStream2,
            constant_scope: bool,
        ) -> TokenStream2 {
            let Self {
                if_token,
                sub_tree,
                else_arm,
                condition,
                ..
            } = self;

            let else_arm = else_arm.to_token_stream(
                branch_index + 1,
                node_index,
                max_node_height,
                parent_return_type,
                constant_scope,
            );

            let sub_tree = {
                let mut tokens = TokenStream2::new();
                sub_tree.to_tokens(
                    &mut tokens,
                    branch_index,
                    node_index,
                    max_node_height,
                    parent_return_type,
                );
                tokens
            };

            quote! {
                #if_token #condition {
                    #sub_tree
                } #else_arm
            }
        }
    }
}

mod else_arm {
    use super::if_arm::IfArm;
    use crate::conditional::{IfSubBlock, IfSubTree};
    use crate::optional_parse::OptionalParse;
    use quote::quote;
    use syn::__private::TokenStream2;
    use syn::parse::Parse;

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
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use crate::{
        conditional::{if_block::else_arm::ElseArm, IfBlock},
        scope::Scope,
    };

    use anyhow::{bail, ensure};

    #[test]
    fn max_pre_allocations() -> syn::Result<()> {
        Ok(())
    }

    #[test]
    fn conditional_if_block() -> anyhow::Result<()> {
        let const_condition_expr = quote! { 3 == 4};
        let condition_expr = quote! { t == 3 && #const_condition_expr };
        {
            let expr = quote! { if #condition_expr { div } else { a } };

            let IfBlock {
                if_arm,
                scope,
                max_node_height,
            } = syn::parse2(expr)?;

            ensure!(scope == Scope::Observable(vec![quote! {t}]));
            ensure!(if_arm.condition.to_token_stream().to_string() == condition_expr.to_string());
            ensure!(matches!(*if_arm.else_arm, ElseArm::PureArm { .. }));
            ensure!(max_node_height == 1);
        }
        {
            let expr = quote! { if #condition_expr { div } else if #const_condition_expr { a } };

            let IfBlock {
                if_arm,
                scope,
                max_node_height,
            } = syn::parse2(expr)?;

            ensure!(scope == Scope::Observable(vec![quote! {t}]));
            ensure!(if_arm.condition.to_token_stream().to_string() == condition_expr.to_string());
            ensure!(max_node_height == 1);

            let else_arm = &*if_arm.else_arm;
            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
                ensure!(if_arm.scope == Scope::Constant);
                ensure!(
                    if_arm.condition.to_token_stream().to_string()
                        == const_condition_expr.to_string()
                );
                ensure!(matches!(*if_arm.else_arm, ElseArm::None));
            } else {
                bail!("expected ElseArm::WithIfArm")
            }
        }
        {
            let expr = quote! { if #condition_expr { div } else if const #const_condition_expr { a } else { a } };

            let IfBlock {
                if_arm,
                scope,
                max_node_height,
            } = syn::parse2(expr)?;

            ensure!(scope == Scope::Observable(vec![quote! {t}]));
            ensure!(if_arm.condition.to_token_stream().to_string() == condition_expr.to_string());
            ensure!(max_node_height == 1);

            let else_arm = &*if_arm.else_arm;
            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
                ensure!(if_arm.scope == Scope::Constant);
                ensure!(
                    if_arm.condition.to_token_stream().to_string()
                        == const_condition_expr.to_string()
                );
                ensure!(matches!(*if_arm.else_arm, ElseArm::PureArm { .. }));
            } else {
                bail!("expected ElseArm::WithIfArm")
            }
        }
        Ok(())
    }
}
