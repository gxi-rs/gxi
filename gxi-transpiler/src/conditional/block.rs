use quote::ToTokens;

use crate::{
    conditional::IfBlock,
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
};

use super::MatchBlock;
use syn::__private::TokenStream2;

#[doc(include_str = "./README.md")]
pub enum ConditionalBlock {
    If(IfBlock),
    Match(MatchBlock),
}

impl OptionalParse for ConditionalBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        Ok(if let Some(p) = IfBlock::optional_parse(input)? {
            Some(Self::If(p))
        } else if let Some(p) = MatchBlock::optional_parse(input)? {
            Some(Self::Match(p))
        } else {
            None
        })
    }
}

impl_parse_for_optional_parse!(ConditionalBlock);

impl ConditionalBlock {
    pub fn to_token_stream(&self, index: usize) -> TokenStream2 {
        match self {
            ConditionalBlock::If(if_block) => if_block.to_token_stream(index),
            ConditionalBlock::Match(match_block) => match_block.to_token_stream(),
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;

    use crate::conditional::ConditionalBlock;

    #[test]
    fn conditional() -> syn::Result<()> {
        {
            let p = syn::parse2(quote! {
                if k == 2 {
                    div [

                    ]
                }
            })?;
            assert_eq!(
                if let ConditionalBlock::If(_) = p {
                    true
                } else {
                    false
                },
                true
            );
        }
        {
            let p = syn::parse2(quote! {
                match k {
                    2 => {
                        div [

                        ]
                    }
                    _ => ()
                }
            })?;
            assert_eq!(
                if let ConditionalBlock::Match(_) = p {
                    true
                } else {
                    false
                },
                true
            );
        }
        Ok(())
    }
}