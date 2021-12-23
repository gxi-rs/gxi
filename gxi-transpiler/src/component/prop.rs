use crate::scope::Scope;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::Token;

/// list of comma separated props inside parenthesis
#[derive(Default)]
pub struct NodeProps {
    pub props: Vec<NodeProp>,
}

impl Parse for NodeProps {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut this = Self::default();
        // parse props
        if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(input) {
            while !content.is_empty() {
                let prop: NodeProp = content.parse()?;
                this.props.push(prop);
                if !content.is_empty() {
                    content.parse::<syn::token::Comma>()?;
                }
            }
        }

        Ok(this)
    }
}

pub struct NodeProp {
    pub left: Box<syn::Expr>,
    pub right: Box<syn::Expr>,
    pub scope: Scope,
}

impl Parse for NodeProp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // check for * used to mark sccope to be OPen
        let mut scope = Scope::default();

        // TODO: add to doc, const

        let const_tt = input.parse::<Token!(const)>();

        let syn::ExprAssign { left, right, .. } = input.parse()?;

        if const_tt.is_err() {
            scope = Scope::find_expr_scope(&right)?;
        }

        Ok(Self { left, scope, right })
    }
}

impl NodeProp {
    pub fn to_tokens(
        &self,
        tokens: &mut quote::__private::TokenStream,
        return_type: &TokenStream2,
    ) {
        let Self { left, right, scope } = self;

        tokens.append_all(scope.to_token_stream(
            &quote! {
                __node.#left(#right);
            },
            return_type,
        ));
    }
}
