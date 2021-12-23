use std::ops::{Deref, DerefMut};

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
        // conditional block requires node to be converted into strong node type
        // therefore nodes after that need to deref it
        let mut node_is_strong = false;
        // expected index of node in tree
        let mut node_index = 0usize;

        let to_strong_node_type_tokens = quote! {
            let __node = __node.into_strong_node_type();
        };

        let mut consecutive_buffer = ConsecutiveBuffers::new(parent_return_type);

        for block in &self.blocks {
            let mut block_tokens = TokenStream2::new();
            block.to_tokens(&mut block_tokens, node_index);

            // select and prepare buffer
            match block {
                Block::Conditional(conditional_block) => match conditional_block {
                    ConditionalBlock::If(if_block) => {
                        // make __node strong
                        match if_block.scope {
                            Scope::Observable(_) => {
                                consecutive_buffer.swap(tokens, ConsecutiveBufferType::IfTrees);

                                if !node_is_strong {
                                    tokens.append_all(quote! {#to_strong_node_type_tokens});
                                    node_is_strong = true;
                                }

                                consecutive_buffer.append_all(block_tokens);
                            }
                            _ => {
                                if node_is_strong {
                                    consecutive_buffer
                                        .swap(tokens, ConsecutiveBufferType::StrongNodes);
                                }
                                consecutive_buffer.append_all(block_tokens);
                            }
                        }
                    }
                    _ => todo!(),
                },
                Block::Node(_) => {
                    if node_is_strong {
                        consecutive_buffer.swap(tokens, ConsecutiveBufferType::StrongNodes);
                    }
                    consecutive_buffer.append_all(quote! {
                        __node.push(
                           Some(#block_tokens)
                        );
                    });
                }

                _ => consecutive_buffer.append_all(block_tokens),
            };

            match block {
                Block::Execution(_) => (),
                _ => node_index += 1,
            }
        }

        consecutive_buffer.flush(tokens);

        if !node_is_strong {
            tokens.append_all(to_strong_node_type_tokens);
        }
    }
}

struct ConsecutiveBuffers<'a> {
    buffer: TokenStream2,
    buffer_type: ConsecutiveBufferType,
    downcast_type: &'a TokenStream2,
}

impl<'a> ConsecutiveBuffers<'a> {
    fn new(downcast_type: &'a TokenStream2) -> Self {
        Self {
            buffer: TokenStream2::new(),
            buffer_type: ConsecutiveBufferType::None,
            downcast_type,
        }
    }

    fn swap(&mut self, target_buffer: &mut TokenStream2, to: ConsecutiveBufferType) {
        if self.buffer_type != to {
            self.flush(target_buffer);
            self.buffer_type = to;
        }
    }

    fn flush(&mut self, target_buffer: &mut TokenStream2) {
        self.buffer_type
            .flush(&mut self.buffer, target_buffer, &self.downcast_type);
    }
}

#[derive(PartialEq)]
enum ConsecutiveBufferType {
    IfTrees,
    StrongNodes,
    None,
}

impl ConsecutiveBufferType {
    fn flush(
        &self,
        consecutive_buffer: &mut TokenStream2,
        target_buffer: &mut TokenStream2,
        parent_return_type: &TokenStream2,
    ) {
        if consecutive_buffer.is_empty() {
            return;
        }

        let foo = match &self {
            Self::IfTrees => {
                quote! {
                    {
                        #consecutive_buffer
                    }
                }
            }
            Self::StrongNodes => {
                quote! {
                    {
                        use std::ops::DerefMut;
                        let mut __node = __node.as_ref().borrow_mut();
                        let __node =
                            __node.deref_mut().as_mut().downcast_mut::<#parent_return_type>().unwrap();

                        #consecutive_buffer
                    }
                }
            }
            _ => quote! { #consecutive_buffer },
        };

        target_buffer.append_all(foo);
        *consecutive_buffer = TokenStream2::new();
    }
}

impl<'a> Deref for ConsecutiveBuffers<'a> {
    type Target = TokenStream2;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<'a> DerefMut for ConsecutiveBuffers<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

#[cfg(test)]
mod punctuated_blocks_to_tokens {
    use crate::{blocks::Blocks, conditional::IfBlock};
    use quote::{quote, ToTokens};
    use syn::__private::TokenStream2;

    #[test]
    fn chained_multi_if_blocks_without_trailing_else() -> syn::Result<()> {
        let return_type = quote! {gxi::Element};

        let div_tokens = quote! {
            __node.push({
               use gxi::{VNode, VContainerWidget};
               let mut __node = gxi::Element::from_str("div");
               let __node = __node.into_strong_node_type();
               __node
           });
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
                if_block.to_tokens(&mut tokens, 0);

                quote! {
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

                first_if_block.to_tokens(&mut tokens, 1);
                second_if_block.to_tokens(&mut tokens, 2);

                quote! {
                    #div_tokens
                    let __node = __node.into_strong_node_type();
                    {
                        #tokens
                    }
                    {
                        use std::ops::DerefMut;
                        let mut __node = __node.as_ref().borrow_mut();
                        let __node =
                            __node.deref_mut().as_mut().downcast_mut::<#return_type>().unwrap();
                        #div_tokens
                    }
                }
            };

            println!("{}", blocks.to_string());
            println!("{}", expected.to_string());
            assert_eq!(blocks.to_string(), expected.to_string());
        }

        Ok(())
    }
}
