use quote::{ToTokens, TokenStreamExt};

use crate::{component::NodeBlock, execution::ExecutionBlock, init_type::InitType};

pub enum Block {
    ComponentBlock(NodeBlock),
    ExecutionBlock(ExecutionBlock),
    ConditionalBlock,
    IterBlock,
}

impl syn::parse::Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::parse(&input, InitType::Child)? {
            return Ok(Self::ComponentBlock(comp));
        }

        Err(syn::Error::new(input.span(), "didn't expect this here"))
    }
}

impl ToTokens for Block {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            _ => {},
        }
        todo!()
    }
}

/// comma separated multiple blocks
#[derive(Default)]
pub struct BlockParser {
    pub blocks: Vec<Block>,
}

impl syn::parse::Parse for BlockParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut blocks = Vec::default();

        loop {
            if input.is_empty() {
                break;
            }

            blocks.push(input.parse()?);

            if input.parse::<syn::Token![,]>().is_err() {
                break;
            }
        }

        Ok(Self { blocks })
    }
}

impl ToTokens for BlockParser {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
       //append
    }
}
