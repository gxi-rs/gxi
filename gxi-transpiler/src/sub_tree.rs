use std::ops::{Deref, DerefMut};

use syn::parse::Parse;

/// Comma separated Tokens
pub struct SubTree<B: Parse>(pub Vec<B>);

impl<B: Parse> Default for SubTree<B> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<B: Parse> syn::parse::Parse for SubTree<B> {
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

impl<B: Parse> Deref for SubTree<B> {
    type Target = Vec<B>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<B: Parse> DerefMut for SubTree<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
