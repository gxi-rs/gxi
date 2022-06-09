use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

use crate::{
    observables::Observables,
    observer_builder::ObserverBuilder,
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    state::State,
    sub_tree::SubTreeEnumeratorState,
};

use super::arm::{ElseArm, IfArm};

/// # Syntax
///
/// ```
/// $if_arm $else_arm?
/// ```
///
/// > refer to [`IfArm`]
pub struct IfBlock {
    pub if_arm: IfArm,
    pub else_arms: Vec<ElseArm>,
    /// State of the whole tree (conditions + subtree)
    ///
    /// if at least one observable is found **inside the conditions** of any `if_arm`,
    /// the tree is considered as [`State::Observable`], i.e the tree will be
    /// re constructed from the ground up and contexts destroyed if the observable
    /// changes its value.
    /// Obviously, optimizations are put in place to prevent unnecessary re renders.
    /// Note: The ObserverBuilder will not borrow the RefCell value.
    pub state: State,
}

impl OptionalParse for IfBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let mut observables = Observables::default();

        let if_arm = if let Some(if_arm) = IfArm::optional_parse(input, &mut observables)? {
            if_arm
        } else {
            return Ok(None);
        };

        let mut else_arms = Vec::default();

        loop {
            let else_arm = ElseArm::parse(input, &mut observables)?;
            else_arms.push(else_arm);

            if let ElseArm::None = else_arms.last().unwrap() {
                break;
            }
        }

        // only if there is a observable **condition**
        let state = if !observables.is_empty() {
            let mut state = State::Observable(observables);

            state
        } else {
            State::Constant
        };

        Ok(Some(Self {
            if_arm,
            else_arms,
            state,
        }))
    }
}

impl_parse_for_optional_parse!(IfBlock);

/// TODO: tokenization rule
/// TODO: continue here
impl IfBlock {
    pub fn to_tokens(&self, tokens: &mut TokenStream2, _enumerator_state: &SubTreeEnumeratorState) {
        let constant_state = self.state.is_const();

        let if_block_tokens = {
            let mut tokens = TokenStream2::new();

            if !self.if_arm.observables_state_range.is_empty() {
                tokens.append_all(quote! {
                    use std::ops::{DerefMut, Deref};
                });
            }

            tokens.append_all(self.if_arm.to_token_stream(1));

            for (index, else_arm) in self.else_arms.iter().enumerate() {
                tokens.append_all(else_arm.to_token_stream(index + 2, constant_state));
            }

            tokens
        };

        tokens.append_all(self.state.to_token_stream(&ObserverBuilder {
            pre_add_observer_tokens: &quote! {
                let mut __ctx = gxi::IndexedContext::default();
            },
            add_observer_body_tokens: &if_block_tokens,
            borrow: false,
        }))
    }
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use super::{super::arm::ElseArm, IfBlock};
    use crate::observables::Observables;
    use crate::state::State;

    use anyhow::{bail, ensure};

    #[test]
    fn state_test() -> anyhow::Result<()> {
        todo!()
    }

    #[test]
    fn parse_test() -> anyhow::Result<()> {
        let const_condition_expr = quote! { 3 == 4};
        let observable_condition_expr =
            quote! { (t == 3 && #const_condition_expr && 2 < 3 && t == t && t < t2 && t3 < t1) };
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
                else_arms,
                state,
            } = syn::parse2(expr)?;

            ensure!(
                if_arm.condition.to_token_stream().to_string()
                    == observable_condition_expr.to_string()
            );
            ensure!(else_arms.len() == 1);
            if let Some(ElseArm::PureArm { body, .. }) = else_arms.last() {
            } else {
                bail!("")
            }
            ensure!(state == observable_condition_expr_scope);
        }

        Ok(())
    }

    //    #[test]
    //    fn conditional_if_block() -> anyhow::Result<()> {
    //        {
    //            let expr = quote! { if #observable_condition_expr { div } else if #const_condition_expr { a } };
    //
    //            let IfBlock {
    //                if_arm,
    //                state: scope,
    //                max_node_height,
    //            } = syn::parse2(expr)?;
    //
    //            ensure!(scope == observable_condition_expr_scope);
    //            ensure!(
    //                if_arm.condition.to_token_stream().to_string()
    //                    == observable_condition_expr.to_string()
    //            );
    //            ensure!(max_node_height == 1);
    //
    //            let else_arm = &*if_arm.else_arm;
    //            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
    //                ensure!(if_arm.state == State::Constant);
    //                ensure!(
    //                    if_arm.condition.to_token_stream().to_string()
    //                        == const_condition_expr.to_string()
    //                );
    //                ensure!(matches!(*if_arm.else_arm, ElseArm::None));
    //            } else {
    //                bail!("expected ElseArm::WithIfArm")
    //            }
    //        }
    //        {
    //            let expr = quote! { if #observable_condition_expr { div } else if #const_condition_expr { a } else if #observable_condition_expr { a } };
    //
    //            let IfBlock {
    //                if_arm,
    //                state: scope,
    //                max_node_height,
    //            } = syn::parse2(expr)?;
    //
    //            ensure!(scope == observable_condition_expr_scope);
    //            ensure!(
    //                if_arm.condition.to_token_stream().to_string()
    //                    == observable_condition_expr.to_string()
    //            );
    //            ensure!(max_node_height == 1);
    //
    //            let else_arm = &*if_arm.else_arm;
    //            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
    //                ensure!(if_arm.state == State::Constant);
    //                ensure!(
    //                    if_arm.condition.to_token_stream().to_string()
    //                        == const_condition_expr.to_string()
    //                );
    //                if let ElseArm::WithIfArm { if_arm, .. } = &*if_arm.else_arm {
    //                    //                    let mut expected_scope = observable_condition_expr_scope.clone();
    //                    //                    if let Scope::Observable(expected_scope) = &mut expected_scope {
    //                    //                        expected_scope.push(quote! {t4})
    //                    //                    } else {
    //                    //                        unreachable!()
    //                    //                  }
    //                    ensure!(if_arm.state == observable_condition_expr_scope);
    //                    ensure!(
    //                        if_arm.condition.to_token_stream().to_string()
    //                            == quote! {#observable_condition_expr && t4}.to_string()
    //                    );
    //                }
    //                ensure!(matches!(*if_arm.else_arm, ElseArm::PureArm { .. }));
    //            } else {
    //                bail!("expected ElseArm::WithIfArm")
    //            }
    //        }
    //        Ok(())
}
