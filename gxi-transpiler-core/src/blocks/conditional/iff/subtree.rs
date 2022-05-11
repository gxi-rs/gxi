use std::ops::{Deref, DerefMut};

use crate::{
    blocks::node::NodeSubBlock,
    lifetime::LifeTime,
    sub_tree::{NodeSubTreeExt, SubTree},
};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;

#[derive(Default)]
pub struct IfSubTree(pub Vec<NodeSubBlock>);

impl SubTree for IfSubTree {
    type SubBlock = NodeSubBlock;
}

impl NodeSubTreeExt for IfSubTree {}

impl ToTokens for IfSubTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let (token_buff, _) = self.for_each_sub_block(|block, block_tokens, _| {
            if let NodeSubBlock::Node(node) = &block {
                block_tokens.append_all(quote! {
                   //FIX: __node.set_at_index(&__child.as_node(), #node_index);
                });

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
