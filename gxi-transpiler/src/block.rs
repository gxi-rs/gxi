use quote::ToTokens;
use syn::__private::TokenStream2;
use syn::parse::Parse;

use crate::{
    component::NodeBlock, conditional::ConditionalBlock, execution::ExecutionBlock,
    optional_parse::OptionalParse,
};

pub enum Block {
    Node(NodeBlock),
    Execution(ExecutionBlock),
    Conditional(ConditionalBlock),
    #[allow(dead_code)]
    Iter,
}

impl Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::optional_parse(&input)? {
            Ok(Self::Node(comp))
        } else if let Some(ex) = ExecutionBlock::optional_parse(&input)? {
            Ok(Self::Execution(ex))
        } else if let Some(cond) = ConditionalBlock::optional_parse(&input)? {
            Ok(Self::Conditional(cond))
        } else {
            Err(syn::Error::new(input.span(), "unexpected token"))
        }
    }
}

impl Block {
    pub fn to_tokens(&self, tokens: &mut TokenStream2, node_index: usize, parent_return_type: &TokenStream2) {
        match self {
            Block::Node(comp) => comp.to_tokens(tokens),
            Block::Execution(ex) => ex.to_tokens(tokens),
            Block::Conditional(cond) => cond.to_tokens(tokens, node_index, parent_return_type),
            Block::Iter => todo!(),
        }
    }
}

//TODO: add tests for solo conditional and execution blocks tokens
