use proc_macro::TokenTree::Ident;
use std::ops::Deref;

use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{Expr, PathArguments, PathSegment};

use crate::init_type::InitType;

#[doc = include_str ! ("./README.md")]
pub(crate) fn parse_component_block(
    input: &ParseStream,
    init_type: &InitType,
) -> syn::Result<Option<TokenStream2>> {
    if let Ok(mut path) = input.parse::<syn::Path>() {
        let (node, constructor) = {
            let last_segment = path.segments.pop().unwrap().into_value();
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
                0 => {
                    (if last_starts_with_lower_case {
                        (
                            quote! {
                                gxi::Element
                            },
                            quote! {
                                from_str(#name, parent)
                            },
                        )
                    } else {
                        (
                            node,
                            quote! {
                                new(parent)
                            },
                        )
                    })
                }
                // more than 1 then
                _ => {
                    if last_starts_with_lower_case {
                        (
                            path.segments
                                .pop()
                                .unwrap()
                                .into_value()
                                .ident
                                .into_token_stream(),
                            {
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
                            },
                        )
                    } else {
                        (
                            node,
                            quote! {
                                new(parent)
                            },
                        )
                    }
                }
            }
        };

        // parse props
        let mut init_props = TokenStream2::new();
        let mut is_new_props = TokenStream2::new();
        let open_props = TokenStream2::new();

        if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(&input) {
            if !content.is_empty() {
                loop {
                    let syn::ExprAssign { left, right, .. } = content.parse::<syn::ExprAssign>()?;
                    // check for static and dynamic props
                    match right.deref() {
                        syn::Expr::Closure(syn::ExprClosure {
                            asyncness,
                            output,
                            attrs,
                            inputs,
                            body,
                            ..
                        }) => {
                            if asyncness.is_some() {
                                return Err(
                                    syn::Error::new(
                                        content.span(),
                                        "async closures are not supported. Using async update function instead.",
                                    )
                                );
                            }

                            if !output.to_token_stream().is_empty() {
                                return Err(syn::Error::new(
                                    content.span(),
                                    "this closure cannot return a value",
                                ));
                            }

                            is_new_props.append_all(quote! {
                                __node.#left(
                                    #(#attrs)*
                                    move |#inputs| {
                                        #body
                                    }
                                );
                            })
                        }
                        syn::Expr::Lit(syn::ExprLit { lit, .. }) => init_props.append_all(quote! {
                            __node.#left(#lit);
                        }),
                        &_ => {}
                    }

                    if content.is_empty() {
                        break;
                    } else {
                        content.parse::<syn::token::Comma>()?;
                    }
                }
            }
        }

        let props_calls = if is_new_props.is_empty() && open_props.is_empty() {
            TokenStream2::new()
        } else {
            // TODO: replace unwrap with expect clause
            quote! {{
                let mut __node = __node.as_ref().borrow_mut();
                let __node = __node.deref_mut().as_mut().downcast_mut::<#path#node>().unwrap();

                if __is_new {
                    #is_new_props
                }

                #open_props
            }}
        };

        println!("props_call {}", props_calls.to_string());

        // TODO: add expect clause per line
        return Ok(Some(quote! {
            let (__node, __is_new) = init_member(&__node, #init_type, |parent| {
                let __node = #path#node::#constructor;
                #init_props
                __node.into_vnode_type()
            }).unwrap();
            #props_calls
        }));
    }
    Ok(None)
}

enum ExprInitLocation {
    Constructor,
    IfIsNew,
    Open,
}

impl ExprInitLocation {
    fn find(expr: &syn::Expr) -> syn::Result<Self> {
        return match expr {
            Expr::Array(_) => todo!(),
            Expr::AssignOp(_) => todo!(),
            Expr::Binary(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Cast(_) => todo!(),
            Expr::Closure(_) => todo!(),
            Expr::ForLoop(_) => todo!(),
            Expr::If(_) => todo!(),
            Expr::Index(_) => todo!(),
            Expr::Lit(_) => todo!(),
            Expr::Loop(_) => todo!(),
            Expr::While(_) => todo!(),
            Expr::Macro(_) => todo!(),
            Expr::Match(_) => todo!(),
            Expr::MethodCall(_) => todo!(),
            Expr::Paren(_) => todo!(),
            Expr::Path(_) => todo!(),
            Expr::Range(_) => todo!(),
            Expr::Reference(_) => todo!(),
            Expr::Repeat(_) => todo!(),
            Expr::Try(_) => todo!(),
            Expr::TryBlock(_) => todo!(),
            Expr::Tuple(_) => todo!(),
            Expr::Unary(_) => todo!(),
            Expr::Unsafe(_) => todo!(),
            Expr::Assign(_) 
                | Expr::Async(_)
                | Expr::Await(_)
                | Expr::Box(_)
                | Expr::Continue(_)
                | Expr::Group(_)
                | Expr::Let(_)
                | Expr::Struct(_)
                | Expr::Field(_)
                | Expr::Type(_)
                | Expr::Break(_)
                | Expr::Return(_)
                | Expr::Yield(_)
                => {
                Err(syn::Error::new(expr.span(), "didn't expect this here"))
            }
            Expr::Verbatim(_)| Expr::__TestExhaustive(_) => unreachable!(),
        };
    }
}
