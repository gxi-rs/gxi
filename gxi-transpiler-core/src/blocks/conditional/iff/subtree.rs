use std::ops::{Deref, DerefMut};

use crate::{
    blocks::node::NodeSubBlock,
    lifetime::LifeTime,
    sub_tree::{SubTree, SubTreeEnumeratorState},
};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{__private::TokenStream2, parse::Parse};

#[derive(Default)]
pub struct IfSubTree(pub Vec<NodeSubBlock>);

impl SubTree<NodeSubBlock> for IfSubTree {}

impl ToTokens for IfSubTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut enumerator_state = SubTreeEnumeratorState::default();
        let mut token_buff = TokenStream2::new();

        for block in self.0.iter() {
            let mut block_tokens = TokenStream2::new();
            block.to_tokens(&mut block_tokens, &enumerator_state);

            match block {
                NodeSubBlock::Node(node) => {
                    block_tokens.append_all(quote! {
                       //FIX: __node.set_at_index(&__child.as_node(), #node_index);
                    });

                    if let LifeTime::Context(context) = &node.lifetime {
                        block_tokens.append_all(quote! {
                            __ctx.set_value(Box::from(__child));
                        })
                    }
                }
                _ => (),
            }

            token_buff.append_all(block_tokens);
        }

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
