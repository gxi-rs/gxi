use quote::{ToTokens, TokenStreamExt};

use crate::{component::NodeBlock, execution::ExecutionBlock, init_type::InitType};

pub enum Block {
    NodeBlock(NodeBlock),
    ExecutionBlock(ExecutionBlock),
    ConditionalBlock,
    IterBlock,
}

impl syn::parse::Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::parse(&input, InitType::Child)? {
            return Ok(Self::NodeBlock(comp));
        }

        Err(syn::Error::new(input.span(), "didn't expect this here"))
    }
}

impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.append(match self {
            Block::NodeBlock(comp) => comp,
            Block::ExecutionBlock(_ex) => todo!(),
            Block::ConditionalBlock => todo!(),
            Block::IterBlock => todo!(),
        });
    }
}

impl Block {
    pub fn is_serializable(self) -> bool {
        match self {
            Block::NodeBlock(node) => node.serializable,
            Block::ExecutionBlock(_) => false,
            Block::ConditionalBlock => todo!(),
            Block::IterBlock => todo!(),
        }
    }
}
