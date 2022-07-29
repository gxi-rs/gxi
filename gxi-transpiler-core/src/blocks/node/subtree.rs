use std::ops::{Deref, DerefMut};

use quote::{ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::Parse;

use crate::{
    blocks::{conditional::ConditionalBlock, execution::ExecutionBlock, node::NodeBlock},
    lifetime::{ContextType, LifeTime},
    optional_parse::OptionalParse,
    snippets::{self, VNodeActions},
    state::{State, StateExt},
    sub_tree::{NodeSubTreeExt, SubTree, SubTreeEnumeratorState},
};

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
            if let NodeSubBlock::Node(node) = block {
                VNodeActions::Push.to_tokens(block_tokens);
                if let LifeTime::Context(ContextType::Constant(action)) = &node.lifetime {
                    action.to_tokens(block_tokens);
                }
            }
        });

        if enumerator_state.variable_size_blocks > 0 {
            snippets::DynamicIndex::IndexBuff(enumerator_state.variable_size_blocks)
                .to_tokens(tokens);
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
