use std::ops::{Deref, DerefMut};

use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::Parse;

use crate::{
    blocks::{conditional::ConditionalBlock, execution::ExecutionBlock, node::NodeBlock},
    lifetime::ConstantContextAction,
    observables::Observables,
    optional_parse::OptionalParse,
    state::{State, StateExt},
    sub_tree::{NodeSubTreeExt, SubTree, SubTreeEnumeratorState},
};

/// TODO:
pub enum NodeSubBlock {
    Node(NodeBlock),
    Execution(ExecutionBlock),
    Conditional(ConditionalBlock),
    #[allow(dead_code)]
    Iter,
}

impl Parse for NodeSubBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::optional_parse(&input)? {
            Ok(Self::Node(comp))
        } else if let Some(ex) = ExecutionBlock::optional_parse(&input)? {
            Ok(Self::Execution(ex))
        } else if let Some(cond) = ConditionalBlock::optional_parse(&input)? {
            Ok(Self::Conditional(cond))
        } else {
            Err(syn::Error::new(input.span(), "unexpected token"))
        }
    }
}

impl NodeSubBlock {
    pub fn to_tokens(&self, tokens: &mut TokenStream2, enumerator_state: &SubTreeEnumeratorState) {
        match self {
            NodeSubBlock::Node(comp) => comp.to_tokens(tokens),
            NodeSubBlock::Execution(ex) => ex.to_tokens(tokens),
            NodeSubBlock::Conditional(cond) => cond.to_tokens(tokens, enumerator_state),
            NodeSubBlock::Iter => todo!(),
        }
    }
}

impl StateExt for NodeSubBlock {
    fn get_state(&self) -> State {
        match self {
            Self::Node(node) => node.get_state(),
            Self::Execution(_) | Self::Conditional(_) => State::Constant,
            Self::Iter => todo!(),
        }
    }

    fn get_nested_state(&self) -> State {
        match self {
            Self::Node(node) => node.get_nested_state(),
            Self::Execution(_) | Self::Conditional(_) => State::Constant,
            Self::Iter => todo!(),
        }
    }
}

/// [`SubTree`] of [`NodeSubBlocks`](NodeSubBlock).
///
/// # ToTokens
///
/// ## Index Buff
///
/// if subtree has variable sized blocks then **__index_buff** is
/// used to track variable indexes occupied by it at any given time.
///
/// *e.g*
/// ```rust
/// __index_buff[$variable_size_block_position] = indexes_occupied
/// ```
///
/// ```rust
/// let mut __index_buff = vec[0usize; $number_of_variable_size_blocks]
/// ```
///
#[derive(Default)]
pub struct NodeSubTree(pub Vec<NodeSubBlock>);

impl SubTree for NodeSubTree {
    type SubBlock = NodeSubBlock;
}

impl NodeSubTreeExt for NodeSubTree {}

impl ToTokens for NodeSubTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (token_buff, enumerator_state) = self.for_each_sub_block(|block, block_tokens, _| {
            if let NodeSubBlock::Node(_) = block {
                block_tokens.append_all(quote! {
                    __node.push(&__child.as_node(), &*__child);
                });
                ConstantContextAction::Push.to_tokens(block_tokens);
            }
        });

        if enumerator_state.variable_size_blocks > 0 {
            let number_of_variable_size_blocks = enumerator_state.variable_size_blocks;
            tokens.append_all(quote! {
                let mut __index_buff = vec[0usize; #number_of_variable_size_blocks];
            });
        }

        tokens.append_all(token_buff)
    }
}

impl Deref for NodeSubTree {
    type Target = Vec<NodeSubBlock>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NodeSubTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod punctuated_blocks_to_tokens {
    use crate::blocks::{conditional::IfBlock, node::NodeSubTree};
    use anyhow::ensure;
    use quote::quote;
    use syn::__private::TokenStream2;

    #[test]
    fn chained_multi_if_blocks_without_trailing_else() -> anyhow::Result<()> {
        let return_type = quote! {gxi::Element};

        let div_tokens = quote! {
            __node.push(Some({
               use gxi::{VNode, VContainerWidget};
               let mut __node = gxi::Element::from("div");
               let __node = __node.into_strong_node_type();
               __node
           }));
        };

        {
            let if_block = quote! {
                if state == 2 {
                    div
                } else if const state2 == 3 {
                    div
                }
            };

            let blocks = {
                let blocks: NodeSubTree = syn::parse2(quote! (#if_block))?;

                let mut blocks_buff = TokenStream2::new();
                blocks.to_tokens(&mut blocks_buff);
                blocks_buff
            };

            let expected = {
                let if_block: IfBlock = syn::parse2(if_block)?;

                let mut tokens = TokenStream2::new();
                if_block.to_tokens(&mut tokens, 0);

                quote! {
                    __node.push(None);
                    let __node = __node.into_strong_node_type();
                    {
                        #tokens
                    }
                }
            };

            ensure!(blocks.to_string() == expected.to_string());
        }

        {
            let first_if_block = quote! {
                if state == 2 {
                    div
                } else if const state2 == 3 {
                    div
                }
            };

            let second_if_block = quote! {
                if state2 == 3 {
                    div
                } else {
                    div
                }
            };

            let blocks = {
                let blocks: NodeSubTree = syn::parse2(quote! {
                    div,
                    #first_if_block,
                    #second_if_block,
                    div
                })?;

                let mut blocks_buff = TokenStream2::new();
                blocks.to_tokens(&mut blocks_buff);
                blocks_buff
            };

            let expected = {
                let first_if_block: IfBlock = syn::parse2(first_if_block)?;
                let second_if_block: IfBlock = syn::parse2(second_if_block)?;

                let mut tokens = TokenStream2::new();

                first_if_block.to_tokens(&mut tokens, 1);
                second_if_block.to_tokens(&mut tokens, 2);

                quote! {
                    #div_tokens
                    __node.push(None);
                    __node.push(None);
                    #div_tokens
                    let __node = __node.into_strong_node_type();
                    {
                        #tokens
                    }
                }
            };

            ensure!(blocks.to_string() == expected.to_string());
        }

        Ok(())
    }
}
