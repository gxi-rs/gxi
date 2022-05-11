use std::collections::VecDeque;

use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::Parse;

use crate::blocks::node::NodeBlock;
use crate::lifetime::LifeTime;

use super::node::{NodeSubBlock, NodeWrapper};

/// As the name suggests, root block is the first block of the gxi macro.
/// The root block inherently is a single [`NodeBlock`]
pub struct RootBlock {
    pub root_node_block: NodeBlock,
    /// highest lifetime of the whole tree
    /// gxi::VNodeContext is created accordingly.
    pub tree_life_time: LifeTime,
}

impl Parse for RootBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let root_node_block = NodeBlock::parse(input)?;

        // traverse the tree and check if it requires_context
        let tree_life_time = {
            let mut node_q = VecDeque::from([&root_node_block]);

            loop {
                if let Some(node) = node_q.pop_front() {
                    if let LifeTime::Context(_) = node.lifetime {
                        break LifeTime::Context(Default::default());
                    }

                    for sub_node in node.sub_tree.iter() {
                        if let NodeSubBlock::Node(sub_node) = sub_node {
                            node_q.push_back(sub_node);
                        }
                    }
                } else {
                    break LifeTime::Constant;
                }
            }
        };

        Ok(Self {
            root_node_block,
            tree_life_time,
        })
    }
}

impl ToTokens for RootBlock {
    /// create initial context based on the sub_tree
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let mut head_tokens = quote! {
            use gxi::{VNode, VContainer, VLeaf};
        };

        let (ctx_tokens, ctx_type) = if let LifeTime::Context(_) = self.tree_life_time {
            head_tokens.append_all(quote! {
                let mut __ctx = gxi::ConstContext::default();
            });
            (quote!(,Box::from(__ctx)), quote!(WithCtx))
        } else {
            (quote!(), quote!(NoCtx))
        };

        let vnode_shell_tokens = if let NodeWrapper::Rc = self.root_node_block.wrapper {
            quote!(Rc(__child))
        } else {
            quote!(Default(__child))
        };

        let root_node_block = &self.root_node_block;

        tokens.append_all(quote! {{
            #head_tokens
            #root_node_block
            gxi::VNodeContext::#ctx_type(gxi::VNodeShell::#vnode_shell_tokens #ctx_tokens)
        }});
    }
}

#[cfg(test)]
mod tests {
    use super::RootBlock;
    use anyhow::ensure;
    use quote::quote;

    #[test]
    fn parser_test() -> anyhow::Result<()> {
        //        ensure!(
        //            syn::parse2::<RootBlock>(quote! {
        //                h1 [
        //                   h1 ( state = state_variable )
        //                ]
        //            })?
        //            .component_requires_context
        //        );

        Ok(())
    }
}
