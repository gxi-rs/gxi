use syn::parse::Parse;

pub struct SubTree<B: Parse> {
    pub blocks: Vec<B>,
}

impl<B: Parse> Default for SubTree<B> {
    fn default() -> Self {
        Self { blocks: Default::default() }
    }
}

impl<B: Parse> syn::parse::Parse for SubTree<B> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block = B::parse(input)?;

            this.blocks.push(block);

            if input.is_empty() {
                break;
            } else {
                input.parse::<syn::token::Comma>()?;
            }
        }

        Ok(this)
    }
}
