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
    /// index_buff_index -> should originate from NodeSubTree
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        state: &State,
        parent_enumerator_state: &SubTreeEnumeratorState,
    ) {
        let (token_buff, enumerator_state) =
            self.for_each_sub_block(|block, block_tokens, enumerator_state| {
                if let NodeSubBlock::Node(node) = &block {
                    // refer to `src/vnode.rs
                    if let State::Constant = state {
                        snippets::VNodeActions::Push.to_tokens(block_tokens);
                    } else {
                        snippets::VNodeActions::InsertAtIndex(enumerator_state.index_counter)
                            .to_tokens(block_tokens);
                    }

                    if let LifeTime::Context(_) = &node.lifetime {
                        snippets::IndexedContext::SetValue.to_tokens(block_tokens)
                    }
                }
            });

        snippets::IndexedContext::ComputeIndex(
            parent_enumerator_state.dynamic_places_occupied,
            enumerator_state.index_counter,
            parent_enumerator_state.constant_places_occupied,
        )
        .to_tokens(tokens);
        tokens.append_all(token_buff);
        snippets::VNodeActions::RemoveElements.to_tokens(tokens);
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
