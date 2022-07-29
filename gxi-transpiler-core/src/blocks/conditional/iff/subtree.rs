use std::ops::{Deref, DerefMut};

use crate::{
    blocks::node::NodeSubBlock,
    lifetime::LifeTime,
    snippets,
    state::State,
    sub_tree::{NodeSubTreeExt, SubTree, SubTreeEnumeratorState},
};
use quote::{ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;

// TODO: add deref derive
#[derive(Default)]
pub struct IfSubTree(pub Vec<NodeSubBlock>);

impl SubTree for IfSubTree {
    type SubBlock = NodeSubBlock;
}

impl NodeSubTreeExt for IfSubTree {}

impl IfSubTree {
    pub fn to_tokens(&self, tokens: &mut TokenStream2, state: &State) {
        let (token_buff, _) = self.for_each_sub_block(|block, block_tokens, enumerator_state| {
            if let NodeSubBlock::Node(node) = &block {
                let SubTreeEnumeratorState {
                    // TODO: rename this globally
                    indexes_occupied: index_offset,
                    variable_size_blocks: _,
                } = enumerator_state;

                // refer to `src/vnode.rs
                if let State::Constant = state {
                    snippets::VNodeActions::InsertAtIndex(*index_offset).to_tokens(block_tokens);
                } else {
                    snippets::VNodeActions::Push.to_tokens(block_tokens);
                }

                if let LifeTime::Context(_) = &node.lifetime {
                    snippets::IndexedContext::SetValue.to_tokens(block_tokens)
                }
            }
        });

        tokens.append_all(token_buff)
    }
}

impl Deref for IfSubTree {
    type Target = Vec<NodeSubBlock>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IfSubTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
