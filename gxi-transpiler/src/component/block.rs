use crate::{BlockParser, ComponentProp, InitType};
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;

#[doc = include_str ! ("./README.md")]
pub struct ComponentBlock {
    init_type: InitType,
    props: Vec<ComponentProp>,
    constructor: TokenStream2,
    path: syn::Path,
    children: Vec<crate::Block>,
}

impl ComponentBlock {
    pub fn parse(input: &ParseStream, init_type: InitType) -> syn::Result<Option<Self>> {
        if let Ok(mut path) = input.parse::<syn::Path>() {
            let constructor = {
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
                                .into_token_stream(),
                            )?;
                            quote! {
                                from_str(#last_segment, parent)
                            }
                        } else {
                            quote! {
                                new(parent)
                            }
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
                            quote! { #constructor( #args parent) }
                        } else {
                            quote! {
                                new(parent)
                            }
                        }
                    }
                }
            };

            let props = Vec::default();

            // parse props
            if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(&input) {
                loop {
                    if content.is_empty() {
                        break;
                    }
                    props.push(content.parse()?);
                    if !content.is_empty() {
                        content.parse::<syn::token::Comma>()?;
                    } else {
                        break;
                    }
                }
            }

            // parse children
            let children =
                if let Ok(syn::group::Braces { content, .. }) = syn::group::parse_braces(&input) {
                    if !content.is_empty() {
                        content.parse::<BlockParser>()?.blocks
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
                children,
            }));
        }
        Ok(None)
    }
}
