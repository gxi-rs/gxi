use crate::{Blocks, InitType, Scope};
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;

use super::NodeProps;

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
    pub constructor: TokenStream2,
    pub path: syn::Path,
    pub subtree: Blocks,
    pub props: NodeProps,
    /// highest scope of node, compared to scope of subtree and props
    pub scope: Scope,
}

impl NodeBlock {
    pub fn parse(input: &ParseStream, init_type: &InitType) -> syn::Result<Option<Self>> {
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
                                    from_str(#name, parent)
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

            let mut scope = Scope::default();
            scope.comp_and_promote(&props.scope);
            scope.comp_and_promote(&subtree.scope);

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
        let init_type = &self.init_type;
        let path = &self.path;
        let constructor = &self.constructor;
        let mut const_scope = TokenStream2::new();

        let outer_props = {
            let mut partial_scope = TokenStream2::new();
            let mut open_scope = TokenStream2::new();

            // props
            for prop in &self.props.props {
                match prop.scope {
                    Scope::Constant => const_scope.append_all(quote! {
                        #prop
                    }),
                    Scope::PartialOpen => partial_scope.append_all(quote! {
                        #prop
                    }),
                    Scope::Open => open_scope.append_all(quote! {
                        #prop
                    }),
                }
            }

            if partial_scope.is_empty() && open_scope.is_empty() {
                TokenStream2::new()
            } else {
                // enclose partial_scope inside is_new logic if it is not empty
                if !partial_scope.is_empty() {
                    partial_scope = quote! {
                        if __is_new {
                            #partial_scope
                        }
                    }
                }

                // enclose open_scope inside block
                if !open_scope.is_empty() {
                    open_scope = quote! {
                        {
                            #open_scope
                        }
                    }
                }

                quote! {
                    {
                        use std::ops::DerefMut;

                        let mut __node = __node.as_ref().borrow_mut();
                        let __node = __node.deref_mut().as_mut().downcast_mut::<#path>().unwrap();
                        #partial_scope
                        #open_scope
                    }
                }
            }
        };

        //TODO: implement serialization in case of cfg web amd subtree constant scope, enclosed in
        // a wrapper

        // subtree
        let outer_subtree = {
            let subtree = &self.subtree;

            if subtree.blocks.is_empty() {
                TokenStream2::new()
            } else {
                let subtree = quote! {
                    let __cont = __node.clone();
                    #subtree
                };

                if let Scope::Open = self.subtree.scope {
                    subtree
                } else {
                    quote! {
                        if __is_new {
                            #subtree
                        }
                    }
                }
            }
        };

        // assemble
        tokens.append_all(quote! {
            let (__node, __is_new) = init_member(&__node, #init_type, |parent| {
                let __node = #path::#constructor;
                #const_scope
                __node.into_vnode_type()
            }).unwrap();
            #outer_props
            #outer_subtree
        });
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
