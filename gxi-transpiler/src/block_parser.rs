use crate::Block;
use quote::{ToTokens, TokenStreamExt};

/// comma separated multiple blocks
#[derive(Default)]
pub struct BlockParser {
    pub blocks: Vec<Block>,
    /// serializable if the subtree and the node itself is constant
    pub serializable: bool
}

impl syn::parse::Parse for BlockParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut blocks = Vec::default();
        let mut serializable = true;
        loop {
            if input.is_empty() {
                break;
            }
            
            let block:Block = input.parse()?;
            
            if !block.is_serializable() {
              serializable = false;      
            }

            blocks.push(block);

            if input.parse::<syn::token::Comma>().is_err() {
                break;
            }
        }

        Ok(Self { blocks, serializable })
    }
}

impl ToTokens for BlockParser {
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
