use crate::{block::Block, conditional::ConditionalBlock, scope::Scope};
use quote::{quote, TokenStreamExt};

use syn::__private::TokenStream2;

/// comma separated multiple blocks
/// Blocks can't exist independently they need a __node (impl Node) to operate on
/// __node is converted to StrongNode at the end
#[derive(Default)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

impl syn::parse::Parse for Blocks {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();

        loop {
            if input.is_empty() {
                break;
            }

            let block = Block::parse(input)?;

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

impl Blocks {
    pub fn to_tokens(
        &self,
        tokens: &mut quote::__private::TokenStream,
        parent_return_type: &TokenStream2,
    ) {
        // expected index of node in tree
        let mut node_index = 0usize;

        let mut if_buffer = TokenStream2::new();

        for block in &self.blocks {
            let mut block_tokens = TokenStream2::new();
            block.to_tokens(&mut block_tokens, node_index, parent_return_type);

            // select and prepare buffer
            match block {
                Block::Conditional(conditional_block) => match conditional_block {
                    ConditionalBlock::If(if_block) => {
                        tokens.append_all(quote! {__node.push(None);});
                        // make __node strong
                        match if_block.scope {
                            Scope::Observable(_) => {
                                if_buffer.append_all(block_tokens);
                            }
                            _ => {
                                tokens.append_all(block_tokens);
                            }
                        }
                    }
                    _ => todo!(),
                },
                Block::Node(_) => {
                    tokens.append_all(quote! {
                        __node.push(
                           Some(#block_tokens)
                        );
                    });
                }

                _ => tokens.append_all(block_tokens),
            };

            match block {
                Block::Execution(_) => (),
                _ => node_index += 1,
            }
        }

        if !if_buffer.is_empty() {
            if_buffer = quote! {
                {
                    #if_buffer
                }
            }
        }

        tokens.append_all(quote! {
            let __node = __node.into_strong_node_type();
            #if_buffer
        })
    }
}

#[cfg(test)]
mod punctuated_blocks_to_tokens {
    use crate::{blocks::Blocks, conditional::IfBlock};
    use quote::quote;
    use syn::__private::TokenStream2;

    #[test]
    fn chained_multi_if_blocks_without_trailing_else() -> syn::Result<()> {
        let return_type = quote! {gxi::Element};

        let div_tokens = quote! {
            __node.push(Some({
               use gxi::{VNode, VContainerWidget};
               let mut __node = gxi::Element::from_str("div");
               let __node = __node.into_strong_node_type();
               __node
           }));
        };

        {
            let if_block = quote! {
                if state == 2 {
                    div
                } else if const state2 == 3 {
                    div
                }
            };

            let blocks = {
                let blocks: Blocks = syn::parse2(quote! (#if_block))?;

                let mut blocks_buff = TokenStream2::new();
                blocks.to_tokens(&mut blocks_buff, &return_type);
                blocks_buff
            };

            let expected = {
                let if_block: IfBlock = syn::parse2(if_block)?;

                let mut tokens = TokenStream2::new();
                if_block.to_tokens(&mut tokens, 0, &return_type);

                quote! {
                    __node.push(None);
                    let __node = __node.into_strong_node_type();
                    {
                        #tokens
                    }
                }
            };

            assert_eq!(blocks.to_string(), expected.to_string());
        }

        {
            let first_if_block = quote! {
                if state == 2 {
                    div
                } else if const state2 == 3 {
                    div
                }
            };

            let second_if_block = quote! {
                if state2 == 3 {
                    div
                } else {
                    div
                }
            };

            let blocks = {
                let blocks: Blocks = syn::parse2(quote! {
                    div,
                    #first_if_block,
                    #second_if_block,
                    div
                })?;

                let mut blocks_buff = TokenStream2::new();
                blocks.to_tokens(&mut blocks_buff, &return_type);
                blocks_buff
            };

            let expected = {
                let first_if_block: IfBlock = syn::parse2(first_if_block)?;
                let second_if_block: IfBlock = syn::parse2(second_if_block)?;

                let mut tokens = TokenStream2::new();

                first_if_block.to_tokens(&mut tokens, 1, &return_type);
                second_if_block.to_tokens(&mut tokens, 2, &return_type);

                quote! {
                    #div_tokens
                    __node.push(None);
                    __node.push(None);
                    #div_tokens
                    let __node = __node.into_strong_node_type();
                    {
                        #tokens
                    }
                }
            };

            assert_eq!(blocks.to_string(), expected.to_string());
        }

        Ok(())
    }
}
