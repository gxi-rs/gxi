use std::ops::{Deref, DerefMut};
use std::slice::Iter;

use quote::ToTokens;
use syn::{__private::TokenStream2, parse::Parse};

use crate::blocks::node::NodeSubBlock;

/// Comma separated Tokens
pub trait SubTree:
    Default + ToTokens + Sized + Deref<Target = Vec<Self::SubBlock>> + DerefMut
{
    type SubBlock: Parse;

    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block = Self::SubBlock::parse(input)?;

            this.push(block);

            if input.is_empty() {
                break;
            } else {
                input.parse::<syn::token::Comma>()?;
            }
        }

        Ok(this)
    }
}

pub trait NodeSubTreeExt: SubTree<SubBlock = NodeSubBlock> {
    fn for_each_sub_block<F: Fn(&NodeSubBlock, &mut TokenStream2, &SubTreeEnumeratorState)>(
        &self,
        callback: F,
    ) -> (TokenStream2, SubTreeEnumeratorState) {
        let mut enumerator_state = SubTreeEnumeratorState::default();
        let mut token_buff = TokenStream2::new();

        for block in self.deref().iter() {
            block.to_tokens(&mut token_buff, &enumerator_state);

            match block {
                NodeSubBlock::Conditional(_) | NodeSubBlock::Iter => {
                    enumerator_state.variable_size_blocks += 1;
                }
                NodeSubBlock::Node(_) => {
                    enumerator_state.indexes_occupied += 1;
                }
                _ => (),
            };

            callback(block, &mut token_buff, &enumerator_state)
        }

        (token_buff, enumerator_state)
    }
}

#[derive(Default)]
pub struct SubTreeEnumeratorState {
    pub indexes_occupied: usize,
    pub variable_size_blocks: usize,
}
