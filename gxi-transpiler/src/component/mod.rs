use proc_macro::TokenTree::Ident;

use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::{PathArguments, PathSegment};

use crate::init_type::InitType;

#[doc = include_str ! ("./README.md")]
pub(crate) fn parse_component_block(
    input: &ParseStream,
    init_type: &InitType,
) -> syn::Result<Option<TokenStream2>> {
    if let Ok(mut path) = input.parse::<syn::Path>() {
        let (is_element, node, constructor) = {
            let last_segment = path.segments.pop().unwrap().into_value();
            let node = last_segment.ident.clone();
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
                0 => (
                    last_starts_with_lower_case,
                    node,
                    if last_starts_with_lower_case {
                        quote! {
                            from_str(#name, parent)
                        }
                    } else {
                        quote! {
                            new(parent)
                        }
                    },
                ),
                // more than 1 then
                _ => {
                    if last_starts_with_lower_case {
                        (false, path.segments.pop().unwrap().into_value().ident, {
                            let syn::group::Parens { content, .. } =
                                syn::group::parse_parens(&input)?;
                            let mut args = TokenStream2::new();
                            if !content.is_empty() {
                                loop {
                                    args.append_all(
                                        content.parse::<syn::Expr>()?.into_token_stream(),
                                    );
                                    args.append_all(quote! {,});
                                    // if stream is empty then break
                                    if content.is_empty() {
                                        break;
                                    } else {
                                        // else expect a comma
                                        content.parse::<syn::token::Comma>()?;
                                    }
                                }
                            }
                            quote! { #node( #args parent) }
                        })
                    } else {
                        (
                            false,
                            node,
                            quote! {
                                new(parent)
                            },
                        )
                    }
                }
            }
        };

        let node_path = if is_element {
            quote! {
                gxi::Element::
            }
        } else {
            let mut path = path.to_token_stream();
            if !path.is_empty() {
                path.append_all(quote! {::});
            }
            path
        };

        // TODO: add expect clause per line
        return Ok(Some(quote! {
           let (node, is_new) = init_member(&node, #init_type, |parent| {
               #node_path#node::#constructor.into_vnode_type()
           }).unwrap();
        }));
    }
    Ok(None)
}
