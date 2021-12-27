use super::ConditionalBlock;
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

impl ToTokens for IfSubBlock {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        todo!()
    }
}

pub type IfSubTree = SubTree<IfSubBlock>;

impl ToTokens for IfSubTree {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let mut token_buff = TokenStream2::new();

        for block in self.blocks {
            block.to_tokens(&mut token_buff);
        }

        tokens.append_all(quote! {})
    }
}
