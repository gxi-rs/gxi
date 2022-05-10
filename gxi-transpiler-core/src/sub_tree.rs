use std::ops::{Deref, DerefMut};

use quote::ToTokens;
use syn::parse::Parse;

/// Comma separated Tokens
//pub struct SubTree<B: Parse>(pub Vec<B>);
pub trait SubTree<B: Parse>:
    Default + ToTokens + Sized + Deref<Target = Vec<B>> + DerefMut
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block = B::parse(input)?;

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

#[derive(Default)]
pub struct SubTreeEnumeratorState {
    pub indexes_occupied: usize,
    pub variable_size_blocks: usize,
}
