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

        let mut if_token_tree = TokenStream2::new();

        let mut strong_children = TokenStream2::new();

        for block in &self.blocks {
            match block {
                Block::Conditional(conditional_block) => {
                    if let ConditionalBlock::If(if_block) = conditional_block {
                        if let Scope::Observable(_) = if_block.scope {
                            // append once
                            if !node_is_strong {
                                tokens.append_all(quote! {#to_strong_node_type_tokens});
                                node_is_strong = true;
                            }
                        }
                    }
                }
                _ => {
                    if !if_token_tree.is_empty() {
                        tokens.append_all(quote! {
                            {
                                #if_token_tree
                            }
                        });
                        if_token_tree = TokenStream2::new();
                    }
                }
            }

            let mut block_tokens = TokenStream2::new();
            block.to_tokens(&mut block_tokens, node_index);

            match block {
                Block::Node(_) => tokens.append_all(quote! {
                    __node.push(
                        #block_tokens
                    );
                }),
                Block::Execution(_) => tokens.append_all(block_tokens),
                // conditional
                Block::Conditional(conditional_block) => match conditional_block {
                    ConditionalBlock::If(if_block) => {
                        if let Scope::Observable(_) = if_block.scope {
                            if_token_tree.append_all(block_tokens)
                        } else {
                            tokens.append_all(block_tokens)
                        }
                    }
                    ConditionalBlock::Match(_) => todo!(),
                },
                Block::Iter => todo!(),
            }

            match block {
                Block::Execution(_) => (),
                _ => node_index += 1,
            }
        }

        if !node_is_strong {
            tokens.append_all(to_strong_node_type_tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{blocks::Blocks, gxi};
    use quote::{quote, ToTokens};
    use syn::__private::TokenStream2;

    #[test]
    fn multi_if_block() -> syn::Result<()> {
        let blocks: Blocks = syn::parse2(
            quote! {
                div,
                if state {
                    div
                } else if const state2 {
                    div
                },
                if state2 {
                    div
                },
                if const k == 2 {
                    div
                },
                if state3 {
                    div
                }
            }
            .into(),
        )?;

        let mut buff = TokenStream2::new();
        blocks.to_tokens(&mut buff);

        let div_tokens: TokenStream2 = gxi(quote! {div}.into()).into();

        assert_eq!(
            buff.to_string(),
            quote! {
                let mut __node = gxi::Element::from_str("div");
                __node.push(#div_tokens);
                let __node = __node.to_strong_node_type();
                {
                    {
                        let __if_counter = State::new();
                        state.add_observer(Box::new(move |todos| {
                            use std::ops::DerefMut;
                            let Some(__node) = __node.upgrade() {

                            }
                        }));
                    }
                }
                __node
            }
            .to_string()
        );

        Ok(())
    }
}
