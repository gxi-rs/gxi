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
            if node.requires_context {
                requires_context = true;
                break;
            }

            for sub_node in node.sub_tree.iter() {
                if let NodeSubBlock::Node(sub_node) = sub_node {
                    node_q.push_back(&sub_node);
                }
            }
        }

        let ctx_tokens = if requires_context {
            tokens.append_all(quote! {
                let mut __ctx = ConstContext::default();
            });
            quote!(__ctx)
        } else {
            quote!(())
        };

        self.0.to_tokens(tokens);

        let vnode_shell_tokens = if self.0.requires_rc {
            quote!(Rc(__child))
        } else {
            quote!(Default(Box::from(__child)))
        };

        tokens.append_all(quote! {
            VNodeContext::from(VNodeShell::#vnode_shell_tokens,Some(Box::from(#ctx_tokens)))
        });
    }
}

impl Parse for RootBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(NodeBlock::parse(input)?))
    }
}

#[cfg(test)]
mod tests {
    pub fn foo() {}
}
