use std::ops::{Deref, DerefMut};
use syn::{__private::TokenStream2, parse::Parse};

use crate::blocks::node::NodeSubBlock;
use crate::state::{State, StateExt};

/// Comma separated Tokens
pub trait SubTree: Default + Sized + Deref<Target = Vec<Self::SubBlock>> + DerefMut {
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
    // iterates thorugh each sub block/node block
    fn for_each_sub_block<F: Fn(&NodeSubBlock, &mut TokenStream2, &SubTreeEnumeratorState)>(
        &self,
        callback: F,
    ) -> (TokenStream2, SubTreeEnumeratorState) {
        let mut enumerator_state = SubTreeEnumeratorState::default();
        let mut token_buff = TokenStream2::new();

        for block in self.iter() {
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

    fn get_nested_state(&self) -> State {
        let mut state = State::Constant;

        for block in self.iter() {
            state += block.get_nested_state()
        }

        state
    }
}

#[derive(Default)]
pub struct SubTreeEnumeratorState {
    pub indexes_occupied: usize,
    // number of conditional or iterable blocks, variable size
    pub variable_size_blocks: usize,
}
