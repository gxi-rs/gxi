use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::Parse;

use crate::{
    blocks::{conditional::ConditionalBlock, execution::ExecutionBlock, node::NodeBlock},
    optional_parse::OptionalParse,
    sub_tree::SubTree,
};

pub enum NodeSubBlock {
    Node(NodeBlock),
    Execution(ExecutionBlock),
    Conditional(ConditionalBlock),
    #[allow(dead_code)]
    Iter,
}

impl Parse for NodeSubBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Some(comp) = NodeBlock::optional_parse(&input)? {
            Ok(Self::Node(comp))
        } else if let Some(ex) = ExecutionBlock::optional_parse(&input)? {
            Ok(Self::Execution(ex))
        } else if let Some(cond) = ConditionalBlock::optional_parse(&input)? {
            Ok(Self::Conditional(cond))
        } else {
            Err(syn::Error::new(input.span(), "unexpected token"))
        }
    }
}

impl NodeSubBlock {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        node_blocks: usize,
        parent_return_type: &TokenStream2,
    ) {
        match self {
            NodeSubBlock::Node(comp) => comp.to_tokens(tokens),
            NodeSubBlock::Execution(ex) => ex.to_tokens(tokens),
            NodeSubBlock::Conditional(cond) => {
                cond.to_tokens(tokens, node_blocks, parent_return_type)
            }
            NodeSubBlock::Iter => todo!(),
        }
    }
}

pub type NodeSubTree = SubTree<NodeSubBlock>;

impl NodeSubTree {
    pub fn to_tokens(
        &self,
        tokens: &mut quote::__private::TokenStream,
        parent_return_type: &TokenStream2,
    ) {
        // number of node blocks
        let mut node_blocks = 0usize;
        let mut has_conditional_blocks = false;
        let mut token_buff = TokenStream2::new();

        for block in self.iter() {
            let mut block_tokens = TokenStream2::new();
            block.to_tokens(&mut block_tokens, node_blocks, parent_return_type);

            match block {
                NodeSubBlock::Conditional(_) => {
                    has_conditional_blocks = true;
                }
                NodeSubBlock::Node(node) => {
                    node_blocks += 1;
                    block_tokens.append_all(quote! {
                        __node.push(&__child.as_node(), &*__child);
                    });
                    node.lifetime.to_tokens(&mut block_tokens);
                }
                _ => (),
            };

            token_buff.append_all(block_tokens);
        }

        if has_conditional_blocks {
            tokens.append_all(quote! {
                let mut __extra_nodes_counter = 0usize;
            });
        }

        tokens.append_all(quote! {
            #token_buff
        })
    }
}

#[cfg(test)]
mod punctuated_blocks_to_tokens {
    use crate::{conditional::IfBlock, node::NodeSubTree};
    use anyhow::ensure;
    use quote::quote;
    use syn::__private::TokenStream2;

    #[test]
    fn chained_multi_if_blocks_without_trailing_else() -> anyhow::Result<()> {
        let return_type = quote! {gxi::Element};

        let div_tokens = quote! {
            __node.push(Some({
               use gxi::{VNode, VContainerWidget};
               let mut __node = gxi::Element::from("div");
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
                let blocks: NodeSubTree = syn::parse2(quote! (#if_block))?;

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

            ensure!(blocks.to_string() == expected.to_string());
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
                let blocks: NodeSubTree = syn::parse2(quote! {
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

            ensure!(blocks.to_string() == expected.to_string());
        }

        Ok(())
    }
}
