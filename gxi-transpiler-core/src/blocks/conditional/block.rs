use quote::ToTokens;

use super::{IfBlock, MatchBlock};
use crate::optional_parse::{impl_parse_for_optional_parse, OptionalParse};
use syn::__private::TokenStream2;

/// # Conditional Block
///  
/// ## Syntax
///
///
/// _match expr_
///
/// ```rust
/// match <keyword> expr {
///   ...arms
/// }
/// ```
///
/// **optional `<keyword>`**
///
/// - `const`
///    marks expression as constant i.e not observable.
/// ## Index
///
/// The primary problem with conditional blocks is node indexing.
/// To overcome this problem the concept of relative indexing can be
/// used.
///
/// Each conditional block can produce n number of nodes, which can only
/// be added to parent using `set_at_index` method. The number of nodes
/// produced by any conditional block are added to global `extra_nodes_counter`.
///
///
/// `Resultant Index` = `extra_nodes_counter` + `number of nodes prior to conditional block`
///
/// ```rust
/// {
///     let extra_nodes_counter = 0;
///     // 2 nodes prior
///     if true { // index 3 - 3
///         extra_nodes_counter+=1;
///         let index = 2 + extra_nodes_counter;
///     }
///     // 3 nodes in between
///     if false { // index range 6 - 7
///         
///     }
///     // 5 nodes in between
///     if true { // index range 11 - 13
///
///     }
/// }
/// ```
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
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        node_blocks: usize,
        parent_return_type: &TokenStream2,
    ) {
        match self {
            ConditionalBlock::If(if_block) => {
                if_block.to_tokens(tokens, node_blocks, parent_return_type)
            }
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
