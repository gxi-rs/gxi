use std::collections::HashMap;

use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

use crate::{
    conditional::ConditionalBlock,
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    scope::Scope,
};

use super::{
    arm::{ElseArm, IfArm},
    subtree::IfSubBlock,
};

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

impl IfBlock {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        node_blocks: usize,
        parent_return_type: &TokenStream2,
    ) {
        let mut if_arm_tokens = self.if_arm.to_token_stream(
            1,
            node_blocks,
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

        let mut main_body = self.scope.to_token_stream(&if_arm_tokens);

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

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use super::{super::arm::ElseArm, IfBlock};
    use crate::scope::Scope;

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
