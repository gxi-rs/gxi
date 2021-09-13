use quote::{ToTokens, TokenStreamExt, quote};

use crate::{Block, Scope, init_type::{InitType}};

/// comma separated multiple blocks
#[derive(Default)]
pub struct Blocks {
    pub blocks: Vec<Block>,
    pub scope: Scope,
}

impl syn::parse::Parse for Blocks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        let mut init_type = InitType::Child;

        loop {
            if input.is_empty() {
                break;
            }

            let block = Block::parse(&input, &init_type)?;

            this.scope.comp_and_promote(&block.get_scope());

            this.blocks.push(block);

            if input.is_empty() {
                break;
            } else {
                input.parse::<syn::token::Comma>()?;
            }
            
            init_type = InitType::Sibling;
        }

        Ok(this)
    }
}

impl ToTokens for Blocks {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        for block in &self.blocks {
            tokens.append_all(quote!{
                #block
            });
        }
    }
}

#[cfg(test)]
mod block_parser_tests {
    #[test]
    fn multi_root() {}
}
