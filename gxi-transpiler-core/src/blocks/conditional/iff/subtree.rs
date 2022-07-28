use std::ops::{Deref, DerefMut};

use crate::{
    blocks::node::NodeSubBlock,
    lifetime::LifeTime,
    state::State,
    sub_tree::{NodeSubTreeExt, SubTree, SubTreeEnumeratorState},
};
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

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
                    indexes_occupied,
                    variable_size_blocks: _,
                } = enumerator_state;

                // refer to `src/vnode.rs
                if let State::Constant = state {
                    block_tokens.append_all(quote! {
                       __node.insert_at_index(&__child.as_node(), #indexes_occupied);
                    });
                } else {
                    block_tokens.append_all(quote! {
                       __node.push(&__child.as_node(),  &*__child);
                    });
                }

                if let LifeTime::Context(_) = &node.lifetime {
                    block_tokens.append_all(quote! {
                        __ctx.set_value(Box::from(__child));
                    })
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
