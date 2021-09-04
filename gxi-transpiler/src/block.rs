use crate::{component::ComponentBlock, execution::ExecutionBlock, init_type::InitType};

pub enum Block {
    ComponentBlock(ComponentBlock),
    ExecutionBlock(ExecutionBlock),
    ConditionalBlock,
    IterBlock,
}

#[derive(Default)]
pub struct BlockParser {
    pub blocks: Vec<Block>,
}

impl syn::parse::Parse for BlockParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let blocks = Vec::default();

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

impl syn::parse::Parse for Block {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = ComponentBlock::parse(&input, InitType::Child)? {
            return Ok(Self::ComponentBlock(comp));
        }

        Err(syn::Error::new(input.span(), "didn't expect this here"))
    }
}

impl Block {}
