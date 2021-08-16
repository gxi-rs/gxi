use proc_macro::TokenTree::Ident;

use crate::init_type::InitType;
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::{PathArguments, PathSegment};

#[doc = include_str!("./README.md")]
pub(crate) fn parse_component_block(
    input: &ParseStream,
    init_type: &InitType,
) -> syn::Result<Option<TokenStream2>> {
    if let Ok(mut path) = input.parse::<syn::Path>() {
        println!("Body");
        let (is_element, node, constructor) = {
            let last_segment = path.segments.pop().unwrap().into_value();
            let node = last_segment.ident.clone();
            let name = last_segment.ident.to_string();
            // if there is only one path segment and name starts with a lower case character then
            // it's a NativeElement

            match path.segments.len() {
                // if name starts_with lower case and there is only 1 segment
                // then the node is an element otherwise it is a regular node
                0 => {
                    // check if last_segment starts with a lower case character
                    let starts_with_lower_case = match name.chars().next().unwrap() {
                        'a'..='z' => true,
                        _ => false,
                    };
                    (
                        starts_with_lower_case,
                        node,
                        if starts_with_lower_case {
                            quote! {
                                from_str(#name, parent)
                            }
                        } else {
                            quote! {
                                new(parent)
                            }
                        },
                    )
                }
                // more than 1 then
                _ => {
                    let has_constructor = if let PathArguments::None = &last_segment.arguments {
                        false
                    } else {
                        true
                    };

                    (
                        false,
                        // if last segment has no argument then last segment is the node
                        // otherwise second last segment is the node and last segment is the constructor
                        if has_constructor {
                            node
                        } else {
                            path.segments.last().unwrap().ident.clone()
                        },
                        if has_constructor {
                            last_segment.to_token_stream()
                        } else {
                            quote! {
                                new(parent)
                            }
                        },
                    )
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
