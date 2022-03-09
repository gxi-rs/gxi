use crate::{
    conditional::ConditionalBlock, execution::ExecutionBlock, node::NodeBlock,
    optional_parse::OptionalParse, sub_tree::SubTree,
};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{__private::TokenStream2, parse::Parse};

pub enum IfSubBlock {
    Node(NodeBlock),
    Execution(ExecutionBlock),
    Conditional(ConditionalBlock),
    NoneBlock,
}

impl Parse for IfSubBlock {
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

pub type IfSubTree = SubTree<IfSubBlock>;

impl IfSubBlock {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        node_index: &mut usize,
        max_node_height: usize,
        parent_return_type: &TokenStream2,
    ) {
        match self {
            Self::Node(node) => {
                let ctx_tokens = if node.lifetime.requires_context() {
                    quote! {
                        __ctx.set_value(Box::from(__child));
                    }
                } else {
                    TokenStream2::new()
                };

                tokens.append_all(quote! {
                    #node

                    __node.set_at_index(&*__child, #node_index);
                    #ctx_tokens
                });

                *node_index += 1;
            }
            Self::Execution(ex) => ex.to_tokens(tokens),
            Self::Conditional(ConditionalBlock::If(if_block)) => {
                if_block.to_tokens(tokens, *node_index, parent_return_type);
                *node_index += if_block.max_node_height;
            }
            Self::Conditional(ConditionalBlock::Match(_)) => {
                todo!("match block not yet implemented")
            }
            Self::NoneBlock => {
                for _ in 0..max_node_height {
                    tokens.append_all(quote! {
//                        __node.set_at_index(None, #node_index);
                    });
                    *node_index += 1;
                }
            }
        }
    }
}

impl IfSubTree {
    pub fn to_tokens(
        &self,
        tokens: &mut TokenStream2,
        branch_index: usize,
        mut node_index: usize,
        max_node_height: usize,
        parent_return_type: &TokenStream2,
    ) {
        let mut token_buff = TokenStream2::new();

        let base_node_index = node_index;

        for block in self.iter() {
            block.to_tokens(
                &mut token_buff,
                &mut node_index,
                max_node_height,
                parent_return_type,
            );
        }

        let range = node_index..(base_node_index + max_node_height);
        for _ in range {
            IfSubBlock::NoneBlock.to_tokens(
                &mut token_buff,
                &mut node_index,
                max_node_height,
                parent_return_type,
            )
        }

        tokens.append_all(quote! {
            if __ctx.check_index(#branch_index) {
                #token_buff
            }
        });
    }
}
