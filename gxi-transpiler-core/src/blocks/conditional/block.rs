use quote::ToTokens;

use super::{IfBlock, MatchBlock};
use crate::optional_parse::{impl_parse_for_optional_parse, OptionalParse};
use syn::__private::TokenStream2;

pub enum ConditionalBlock {
    If(Box<IfBlock>),
    Match(MatchBlock),
}

impl OptionalParse for ConditionalBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        Ok(if let Some(p) = IfBlock::optional_parse(input)? {
            Some(Self::If(Box::new(p)))
        } else {
            MatchBlock::optional_parse(input)?.map(Self::Match)
        })
    }
}

impl_parse_for_optional_parse!(ConditionalBlock);

impl ConditionalBlock {
    pub fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            ConditionalBlock::If(if_block) => if_block.to_tokens(tokens),
            ConditionalBlock::Match(match_block) => match_block.to_tokens(tokens),
        }
    }
}

#[cfg(test)]
mod test {
    use quote::quote;

    use super::ConditionalBlock;

    #[test]
    fn conditional_block_match() -> syn::Result<()> {
        {
            let p = syn::parse2(quote! {
                if k == 2 {
                    div [
                        a
                    ]
                }
            })?;
            assert!(matches!(p, ConditionalBlock::If(_)));
        }
        //        {
        //            let p = syn::parse2(quote! {
        //                match k {
        //                    2 => {
        //                        div [
        //
        //                        ]
        //                    }
        //                    _ => ()
        //                }
        //            })?;
        //            assert_eq!(
        //                if let ConditionalBlock::Match(_) = p {
        //                    true
        //                } else {
        //                    false
        //                },
        //                true
        //            );
        //        }
        Ok(())
    }
}
