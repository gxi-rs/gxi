use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

use crate::{
    blocks::conditional::ConditionalBlock,
    observables::Observables,
    observer_builder::ObserverBuilder,
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    state::State,
};

use super::{
    arm::{ElseArm, IfArm},
    subtree::IfSubBlock,
};

pub struct IfBlock {
    /// An `IfBlock` constitutes of linear `IFArms`.
    pub if_arm: IfArm,
    /// if at least one observable is found **inside the conditions** of any `if_arm`,
    /// the tree is considered as `Scope::Observable`, i.e the tree will be
    /// re constructed from the ground up and contexts destroyed if the observable
    /// changes its value.
    /// Obviously, optimizations are put in place to prevent unnecessary re renders.
    /// Note: The ObserverBuilder will not borrow the RefCell value.
    pub state: State,
    /// max possible height of vertical nodes in the nested subtree
    pub max_node_height: usize,
}

impl OptionalParse for IfBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let if_arm = if let Some(if_arm) = IfArm::optional_parse(input)? {
            if_arm
        } else {
            return Ok(None);
        };

        // TODO: get variables from nested sub tree in order to clone them
        let mut scoped_variables = Observables::default();
        let mut max_node_height = 0usize;

        {
            let mut if_arm_ = &if_arm;
            loop {
                let mut max_arm_node_height = 0usize;

                if let State::Observable(observables) = &if_arm.scope {
                    scoped_variables.append(&mut observables.clone());
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

        // remove duplicates
        scoped_variables.remove_duplicates();

        Ok(Some(Self {
            if_arm,
            state: if scoped_variables.is_empty() {
                State::Constant
            } else {
                State::Observable(scoped_variables)
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
    ) {
        let mut if_arm_tokens = self.if_arm.to_token_stream(
            1,
            node_blocks,
            self.max_node_height,
            self.state.is_const(),
        );

        if !self.state.is_const() {
            if_arm_tokens = quote! {
                use std::ops::{DerefMut, Deref};

                #if_arm_tokens
            };
        }

        tokens.append_all(self.state.to_token_stream(&ObserverBuilder {
            pre_add_observer_tokens: &quote! {
                let mut __ctx = gxi::IndexedContext::default();
            },
            add_observer_body_tokens: &if_arm_tokens,
            borrow: false,
        }))
    }
}

impl_parse_for_optional_parse!(IfBlock);

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use super::{super::arm::ElseArm, IfBlock};
    use crate::observables::Observables;
    use crate::state::State;

    use anyhow::{bail, ensure};

    #[test]
    fn max_pre_allocations() -> syn::Result<()> {
        Ok(())
    }

    #[test]
    fn conditional_if_block() -> anyhow::Result<()> {
        let const_condition_expr = quote! { 3 == 4};
        let observable_condition_expr =
            quote! { t == 3 && #const_condition_expr && 2 < 3 && t == t && t < t2 && t3 < t1 };
        let observable_condition_expr_scope = State::Observable(Observables(vec![
            quote! {t},
            quote! {t2},
            quote! {t3},
            quote! {t1},
        ]));

        {
            let expr = quote! { if #observable_condition_expr { div } else { a } };

            let IfBlock {
                if_arm,
                state: scope,
                max_node_height,
            } = syn::parse2(expr)?;

            ensure!(scope == observable_condition_expr_scope);
            ensure!(
                if_arm.condition.to_token_stream().to_string()
                    == observable_condition_expr.to_string()
            );
            ensure!(max_node_height == 1);
        }
        {
            let expr = quote! { if #observable_condition_expr { div } else if #const_condition_expr { a } };

            let IfBlock {
                if_arm,
                state: scope,
                max_node_height,
            } = syn::parse2(expr)?;

            ensure!(scope == observable_condition_expr_scope);
            ensure!(
                if_arm.condition.to_token_stream().to_string()
                    == observable_condition_expr.to_string()
            );
            ensure!(max_node_height == 1);

            let else_arm = &*if_arm.else_arm;
            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
                ensure!(if_arm.scope == State::Constant);
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
            let expr = quote! { if #observable_condition_expr { div } else if #const_condition_expr { a } else if #observable_condition_expr { a } };

            let IfBlock {
                if_arm,
                state: scope,
                max_node_height,
            } = syn::parse2(expr)?;

            ensure!(scope == observable_condition_expr_scope);
            ensure!(
                if_arm.condition.to_token_stream().to_string()
                    == observable_condition_expr.to_string()
            );
            ensure!(max_node_height == 1);

            let else_arm = &*if_arm.else_arm;
            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
                ensure!(if_arm.scope == State::Constant);
                ensure!(
                    if_arm.condition.to_token_stream().to_string()
                        == const_condition_expr.to_string()
                );
                if let ElseArm::WithIfArm { if_arm, .. } = &*if_arm.else_arm {
                    //                    let mut expected_scope = observable_condition_expr_scope.clone();
                    //                    if let Scope::Observable(expected_scope) = &mut expected_scope {
                    //                        expected_scope.push(quote! {t4})
                    //                    } else {
                    //                        unreachable!()
                    //                  }
                    ensure!(if_arm.scope == observable_condition_expr_scope);
                    ensure!(
                        if_arm.condition.to_token_stream().to_string()
                            == quote! {#observable_condition_expr && t4}.to_string()
                    );
                }
                ensure!(matches!(*if_arm.else_arm, ElseArm::PureArm { .. }));
            } else {
                bail!("expected ElseArm::WithIfArm")
            }
        }
        Ok(())
    }
}
