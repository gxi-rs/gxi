use std::ops::{Deref, DerefMut};

use crate::{block::Block, conditional::ConditionalBlock, scope::Scope};
use quote::{quote, ToTokens, TokenStreamExt};

use syn::__private::TokenStream2;

/// comma separated multiple blocks
/// called by NodeBlock
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

impl ToTokens for Blocks {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        // conditional block requires node to be converted into strong node type
        // therefore nodes after that need to deref it
        let mut node_is_strong = false;
        // expected index of node in tree
        let mut node_index = 0usize;

        let to_strong_node_type_tokens = quote! {
            let __node = __node.into_strong_node_type();
        };

        let mut consecutive_buffer = ConsecutiveBuffers::new();

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
                            #block_tokens
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

struct ConsecutiveBuffers {
    buffer: TokenStream2,
    buffer_type: ConsecutiveBufferType,
}

impl ConsecutiveBuffers {
    fn new() -> Self {
        Self {
            buffer: TokenStream2::new(),
            buffer_type: ConsecutiveBufferType::None,
        }
    }

    fn swap(&mut self, target_buffer: &mut TokenStream2, to: ConsecutiveBufferType) {
        if self.buffer_type != to {
            self.flush(target_buffer);
            self.buffer_type = to;
        }
    }

    fn flush(&mut self, target_buffer: &mut TokenStream2) {
        self.buffer_type.flush(&mut self.buffer, target_buffer);
    }
}

#[derive(PartialEq)]
enum ConsecutiveBufferType {
    IfTrees,
    StrongNodes,
    None,
}

impl ConsecutiveBufferType {
    fn flush(&self, consecutive_buffer: &mut TokenStream2, target_buffer: &mut TokenStream2) {
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
                       let mut __node = __node.as_ref().borrow_mut();
                       let __node =
                           __node.deref_mut().as_mut().downcast_mut::<Text>().unwrap();
                    }
                }
            }
            _ => quote! { #consecutive_buffer },
        };

        target_buffer.append_all(foo);
        *consecutive_buffer = TokenStream2::new();
    }
}

impl Deref for ConsecutiveBuffers {
    type Target = TokenStream2;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl DerefMut for ConsecutiveBuffers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

#[cfg(test)]
mod punctuated_blocks_to_tokens {
    use crate::{blocks::Blocks, component::NodeBlock};
    use quote::{quote, ToTokens};
    use syn::__private::TokenStream2;

    #[test]
    fn chained_multi_if_blocks_without_trailing_else() -> syn::Result<()> {
        let div_tokens: NodeBlock = syn::parse2(quote! {gxi})?;
        let div_tokens = div_tokens.to_token_stream();

        {
            let blocks: Blocks = syn::parse2(quote! {
                if state == 2 {
                    div
                } else if const state2 == 3 {
                    div
                },
            })?;

            let mut blocks_buff = TokenStream2::new();
            blocks.to_tokens(&mut blocks_buff);

            assert_eq!(
                blocks_buff.to_string(),
                quote! {
                    let __node = __node.to_strong_node_type();
                    {
                        let __node = std::rc::Rc::downgrade(&__node);
                        let __if_counter = State::new();
                        state.add_observer(Box::new(move |state| {
                            use std::ops::DerefMut;
                            if let Some(__node) = __node.upgrade() {

                                let mut __node = __node.as_ref().borrow_mut();
                                let mut __node = __node
                                    .deref_mut()
                                    .as_mut()
                                    .downcast_mut::<gxi::Element>()
                                    .unwrap();



                                false
                            } else {
                                true
                            }
                        }));
                    }
                    __node
                }
                .to_string()
            );
        }
        {
            let blocks: Blocks = syn::parse2(quote! {
                div,
                if state == 2 {
                    div
                } else if const state2 == 3 {
                    div
                },
                if state2 == 3 {
                    div
                } else {
                    div
                },
                div
            })?;

            let mut blocks_buff = TokenStream2::new();
            blocks.to_tokens(&mut blocks_buff);

            assert_eq!(
                blocks_buff.to_string(),
                quote! {
                    __node.push(#div_tokens);
                    let __node = __node.to_strong_node_type();
                    {
                        let __node = std::rc::Rc::downgrade(&__node);
                        let __if_counter = State::new();
                        state.add_observer(Box::new(move |state| {
                            use std::ops::DerefMut;
                            if let Some(__node) = __node.upgrade() {

                                let mut __node = __node.as_ref().borrow_mut();
                                let mut __node = __node
                                    .deref_mut()
                                    .as_mut()
                                    .downcast_mut::<gxi::Element>()
                                    .unwrap();



                                false
                            } else {
                                true
                            }
                        }));
                    }
                    __node
                }
                .to_string()
            );
        }

        Ok(())
    }
}
