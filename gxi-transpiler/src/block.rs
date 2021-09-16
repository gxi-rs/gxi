use quote::{ToTokens, TokenStreamExt};
use syn::parse::Parse;

use crate::{
    component::{NodeBlock, Scope},
    execution::ExecutionBlock,
    init_type::InitType,
};

pub enum Block {
    NodeBlock(NodeBlock),
    ExecutionBlock(ExecutionBlock),
    ConditionalBlock,
    IterBlock,
}

impl Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Self::parse(input, &InitType::Child(false))
    }
}

impl Block {
    pub fn parse(input: syn::parse::ParseStream, init_type: &InitType) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::parse(&input, init_type)? {
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
        tokens.append_all(match self {
            Block::NodeBlock(comp) => comp.to_token_stream(),
            Block::ExecutionBlock(ex) => ex.to_token_stream(),
            Block::ConditionalBlock => todo!(),
            Block::IterBlock => todo!(),
        });
    }
}

impl Block {
    pub fn get_scope(&self) -> Scope {
        match self {
            Block::NodeBlock(node) => node.scope.clone(),
            //WARN: change this, put more thought
            Block::ExecutionBlock(_) => Scope::PartialOpen,
            Block::ConditionalBlock => todo!(),
            Block::IterBlock => todo!(),
        }
    }
}
