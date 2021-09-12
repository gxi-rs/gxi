use crate::{Block, Scope};

/// comma separated multiple blocks
#[derive(Default)]
pub struct Blocks {
    pub blocks: Vec<Block>,
    pub scope: Scope,
}

impl syn::parse::Parse for Blocks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block: Block = input.parse()?;

            this.scope.comp_and_promote(&block.get_scope());

            this.blocks.push(block);

            if input.parse::<syn::token::Comma>().is_err() {
                break;
            }
        }

        Ok(this)
    }
}

#[cfg(test)]
mod block_parser_tests {
    #[test]
    fn multi_root() {}
}
