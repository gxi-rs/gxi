use crate::{Blocks, ComponentProp, InitType, Scope};
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;

pub enum NodeType {
    // gxi::Element
    Element(String),
    Others,
}

impl Default for NodeType {
    fn default() -> Self {
        Self::Others
    }
}

#[doc = include_str ! ("./README.md")]
pub struct NodeBlock {
    pub node_type: NodeType,
    pub init_type: InitType,
    pub props: Vec<ComponentProp>,
    pub constructor: TokenStream2,
    pub path: syn::Path,
    pub subtree: Blocks,
    pub scope: Scope,
}

impl NodeBlock {
    pub fn parse(input: &ParseStream, init_type: InitType) -> syn::Result<Option<Self>> {
        if let Ok(mut path) = input.parse::<syn::Path>() {
            let (constructor, node_type) = {
                let last_segment = path.segments.last().unwrap();
                let node = last_segment.ident.clone().into_token_stream();
                let name = last_segment.ident.to_string();
                // if there is only one path segment and name starts with a lower case character then
                // it's a NativeElement

                // check if last_segment starts with a lower case character
                let last_starts_with_lower_case = match name.chars().next().unwrap() {
                    'a'..='z' => true,
                    _ => false,
                };

                match path.segments.len() {
                    // if name starts_with lower case and there is only 1 segment
                    // then the node is an element otherwise it is a regular node
                    1 => {
                        if last_starts_with_lower_case {
                            path = syn::parse(
                                quote! {
                                    gxi::Element
                                }
                                .into_token_stream()
                                .into(),
                            )?;
                            (
                                quote! {
                                    from_str(#node, parent)
                                },
                                NodeType::Element(name),
                            )
                        } else {
                            (
                                quote! {
                                    new(parent)
                                },
                                Default::default(),
                            )
                        }
                    }
                    // more than 1 then
                    _ => {
                        if last_starts_with_lower_case {
                            let constructor = path.segments.pop();
                            let syn::group::Parens { content, .. } =
                                syn::group::parse_parens(&input)?;
                            let mut args = TokenStream2::new();
                            loop {
                                if content.is_empty() {
                                    break;
                                }
                                args.append_all(content.parse::<syn::Expr>()?.into_token_stream());
                                args.append_all(quote! {,});
                                // if stream is empty then break
                                if content.is_empty() {
                                    break;
                                } else {
                                    // else expect a comma
                                    content.parse::<syn::token::Comma>()?;
                                }
                            }
                            (quote! { #constructor( #args parent) }, Default::default())
                        } else {
                            (
                                quote! {
                                    new(parent)
                                },
                                Default::default(),
                            )
                        }
                    }
                }
            };

            let mut props = Vec::default();
            let mut scope = Scope::default();
            // parse props
            if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(&input) {
                loop {
                    if content.is_empty() {
                        break;
                    }
                    let prop: ComponentProp = content.parse()?;
                    scope.comp_and_promote(&prop.scope);
                    props.push(prop);
                    if !content.is_empty() {
                        content.parse::<syn::token::Comma>()?;
                    } else {
                        break;
                    }
                }
            }

            // parse children
            let subtree =
                if let Ok(syn::group::Braces { content, .. }) = syn::group::parse_braces(&input) {
                    if !content.is_empty() {
                        content.parse::<Blocks>()?
                    } else {
                        Default::default()
                    }
                } else {
                    Default::default()
                };

            return Ok(Some(Self {
                init_type: init_type.clone(),
                props,
                constructor,
                path,
                node_type,
                scope,
                subtree,
            }));
        }
        Ok(None)
    }
}

/// Optimization Rules:
/// 1. If a component consists of a serializable sub tree then serialize them to string
///
impl ToTokens for NodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        //TODO: component optimization goes here
    }
}

impl ToString for NodeBlock {
    /// if to string is called it means that the whole sub tree is serializable
    fn to_string(&self) -> String {
        //TODO: serialize only if cfg!(feature = "web")
        todo!()
    }
}

//TODO: unit tests about serialize, token sub tree
