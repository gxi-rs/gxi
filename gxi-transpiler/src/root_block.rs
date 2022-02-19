use std::collections::VecDeque;

use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::Parse;

use crate::node::{NodeBlock, NodeSubBlock};

/// the first block of gxi! macro
pub struct RootBlock(pub NodeBlock);

impl ToTokens for RootBlock {
    /// create initial context based on the sub_tree
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        // traverse the tree and check if it requires_context
        let mut requires_context = false;

        let mut node_q = VecDeque::from([&self.0]);

        while let Some(node) = node_q.pop_front() {
            if node.lifetime.requires_context() {
                requires_context = true;
                break;
            }

            for sub_node in node.sub_tree.iter() {
                if let NodeSubBlock::Node(sub_node) = sub_node {
                    node_q.push_back(sub_node);
                }
            }
        }

        let mut head_tokens = quote! {
            use gxi::{VNode, VContainer, VLeaf};
        };

        let (ctx_tokens, ctx_type) = if requires_context {
            head_tokens.append_all(quote! {
                let mut __ctx = gxi::ConstContext::default();
            });
            (quote!(,Box::from(__ctx)), quote!(WithCtx))
        } else {
            (quote!(), quote!(NoCtx))
        };

        let vnode_shell_tokens = if self.0.lifetime.requires_context() {
            quote!(Rc(__child))
        } else {
            quote!(Default(__child))
        };

        let sub_tree = &self.0;

        tokens.append_all(quote! {{
            #head_tokens
            #sub_tree
            gxi::VNodeContext::#ctx_type(gxi::VNodeShell::#vnode_shell_tokens #ctx_tokens)
        }});
    }
}

impl Parse for RootBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(NodeBlock::parse(input)?))
    }
}
