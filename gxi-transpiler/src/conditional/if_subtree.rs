use super::{ConditionalBlock, IfBlock};
use crate::{
    execution::ExecutionBlock, node::NodeBlock, optional_parse::OptionalParse, sub_tree::SubTree,
};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{__private::TokenStream2, parse::Parse};

pub enum IfSubBlock {
    Node(NodeBlock),
    Execution(ExecutionBlock),
    Conditional(ConditionalBlock),
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
        node_index: usize,
        parent_return_type: &TokenStream2,
    ) {
        match self {
            Self::Node(node) => tokens.append_all(quote! {
                __node.push(Some(#node));
            }),
            Self::Execution(ex) => ex.to_tokens(tokens),
            Self::Conditional(cond) => cond.to_tokens(tokens, node_index, parent_return_type),
        }
    }
}

impl ToTokens for IfSubTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut token_buff = TokenStream2::new();

        for block in self.iter() {
            block.to_tokens(&mut token_buff);
        }

        tokens.append_all(quote! {})
    }
}
