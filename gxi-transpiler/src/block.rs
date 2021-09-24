use quote::ToTokens;
use syn::parse::Parse;

use crate::{component::NodeBlock, execution::ExecutionBlock};

pub enum Block {
    NodeBlock(NodeBlock),
    ExecutionBlock(ExecutionBlock),
    ConditionalBlock,
    IterBlock,
}

impl Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::parse(&input)? {
            Ok(Self::NodeBlock(comp))
        } else if let Some(ex) = ExecutionBlock::parse(&input)? {
            Ok(Self::ExecutionBlock(ex))
        } else {
            Err(syn::Error::new(input.span(), "didn't expect this here"))
        }
    }
}

impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            Block::NodeBlock(comp) => comp.to_tokens(tokens),
            Block::ExecutionBlock(ex) => ex.to_tokens(tokens),
            Block::ConditionalBlock => todo!(),
            Block::IterBlock => todo!(),
        }
    }
}
