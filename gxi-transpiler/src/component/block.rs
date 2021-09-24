use super::{NodeProps, Scope};
use crate::blocks::Blocks;
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
    pub constructor: TokenStream2,
    pub path: syn::Path,
    pub subtree: Blocks,
    pub props: NodeProps,
}

impl NodeBlock {
    pub fn parse(input: &ParseStream) -> syn::Result<Option<Self>> {
        if let Ok(mut path) = input.parse::<syn::Path>() {
            let (constructor, node_type) = {
                let last_segment = path.segments.last().unwrap();
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
                                    from_str(#name )
                                },
                                NodeType::Element(name),
                            )
                        } else {
                            (
                                quote! {
                                    new()
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
                            (quote! { #constructor( #args ) }, Default::default())
                        } else {
                            (
                                quote! {
                                    new()
                                },
                                Default::default(),
                            )
                        }
                    }
                }
            };

            let props = input.parse::<NodeProps>()?;

            // parse children
            let subtree = if let Ok(syn::group::Brackets { content, .. }) =
                syn::group::parse_brackets(&input)
            {
                if !content.is_empty() {
                    content.parse::<Blocks>()?
                } else {
                    Default::default()
                }
            } else {
                Default::default()
            };

            return Ok(Some(Self {
                props,
                constructor,
                path,
                node_type,
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
        let Self {
            constructor,
            path,
            subtree,
            props,
            ..
        } = self;

        let mut const_props = TokenStream2::new();
        let mut observable_props = TokenStream2::new();

        for prop in &props.props {
            if let Scope::Constant = prop.scope {
                prop.to_tokens(&mut const_props, path)
            } else {
                prop.to_tokens(&mut observable_props, path)
            }
        }
        // assemble
        tokens.append_all(quote! {{
            let __node = #path::#constructor;
            #subtree

            #const_props

            use gxi::VNode;

            let __node = std::rc::Rc::new(std::cell::RefCell::new(__node.into_vnode_type()));

            #observable_props

            __node
        }});
    }
}

impl ToString for NodeBlock {
    /// if to string is called it means that the whole sub tree is serializable
    fn to_string(&self) -> String {
        if cfg!(feature = "web") {
        } else {
            unreachable!("Can't serialize with the current feature flag. Most likely an internal error. Please use the github issue tracker https://github.com/gxi-rs/gxi/issues")
        }
        todo!()
    }
}

//TODO: unit tests about serialize, token sub tree
