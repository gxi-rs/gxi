use quote::*;
use syn::*;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

pub struct IffParser {
    pub tree: TokenStream2,
}

impl Parse for IffParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let var_name = input.parse::<Ident>()?;
        let in_ident = input.parse::<syn::token::In>()?;
        let stmt = input.parse::<Expr>()?;
        let block_stmts = input.parse::<Block>()?.stmts;
        let tree = quote! {
            let node = {
                let mut top_node = node.clone();
                for #var_name #in_ident #stmt {
                    let node = top_node.clone();
                    {
                        #(#block_stmts)*
                        top_node = node.clone();
                    }
                }
                top_node
            };
        };
        Ok(IffParser { tree })
    }
}
