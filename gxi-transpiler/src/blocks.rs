use crate::{Block, Scope};
use quote::{ToTokens, TokenStreamExt};

/// comma separated multiple blocks
#[derive(Default)]
pub struct Blocks {
    pub blocks: Vec<Block>,
    pub scope: Scope,
}

impl syn::parse::Parse for Blocks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block: Block = input.parse()?;

            this.scope.comp_and_promote(&block.get_scope());

            this.blocks.push(block);

            if input.parse::<syn::token::Comma>().is_err() {
                break;
            }
        }

        Ok(this)
    }
}

impl ToTokens for Blocks {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        //just append all the blocks
        for x in &self.blocks {
            tokens.append_all(x.to_token_stream());
        }
    }
}

#[cfg(test)]
mod block_parser_tests {
    #[test]
    fn multi_root() {}
}
