use quote::ToTokens;
use syn::parse::Parse;

use crate::{
    component::NodeBlock,
    conditional::{ConditionalBlock, MatchBlock},
    execution::ExecutionBlock,
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
            Err(syn::Error::new(input.span(), "didn't expect this here"))
        }
    }
}

impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            Block::Node(comp) => comp.to_tokens(tokens),
            Block::Execution(ex) => ex.to_tokens(tokens),
            Block::Conditional(cond) => cond.to_tokens(tokens),
            Block::Iter => todo!(),
        }
    }
}
