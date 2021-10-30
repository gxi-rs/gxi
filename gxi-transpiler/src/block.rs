use quote::ToTokens;
use syn::parse::Parse;

use crate::{component::NodeBlock, conditional::MatchBlock, execution::ExecutionBlock};

pub enum Block {
    Node(NodeBlock),
    Execution(ExecutionBlock),
    Conditional(MatchBlock),
    #[allow(dead_code)]
    Iter,
}

impl Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::parse(&input)? {
            Ok(Self::Node(comp))
        } else if let Some(ex) = ExecutionBlock::parse(input)? {
            Ok(Self::Execution(ex))
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
