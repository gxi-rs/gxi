use quote::{quote, ToTokens, TokenStreamExt};

use crate::block::Block;

/// comma separated multiple blocks
#[derive(Default)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

impl syn::parse::Parse for Blocks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block = Block::parse(&input)?;

            this.blocks.push(block);

            if input.is_empty() {
                break;
            } else {
                input.parse::<syn::token::Comma>()?;
            }
        }

        Ok(this)
    }
}

impl ToTokens for Blocks {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        for block in &self.blocks {
            if let Block::NodeBlock(_) = block {
                tokens.append_all(quote! {
                    __node.push(
                        #block
                    );
                });
            } else {
                block.to_tokens(tokens);
            }
        }
    }
}

#[cfg(test)]
mod block_parser_tests {
    #[test]
    fn multi_root() {}
}
