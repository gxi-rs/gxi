use quote::*;
use syn::*;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

use crate::init_type::InitType;

pub struct CParser {
    pub tree: TokenStream2,
}

impl CParser {
    fn parse_for_block(input: ParseStream, (pure_index, init_type): (u32, &TokenStream2)) -> TokenStream2 {
        fn for_recursive(input: ParseStream, (pure_index, init_type): (u32, &TokenStream2)) -> TokenStream2 {
            let variable = input.parse::<syn::Ident>().unwrap();
            input.parse::<syn::token::In>().unwrap();
            let for_expr = input.parse::<syn::Expr>().unwrap();
            let content = CParser::custom_parse(input, InitType::Sibling.get_init_type_tuple());
            quote! {
                {
                    c!(#pure_index #init_type Pure);
                    let cont = node.clone();
                    let state_borrow = top_state.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                    let node = {
                       let widget = cont.as_ref().borrow().get_widget_as_container();
                       let mut node_borrow = node.as_ref().borrow_mut();
                       let widget_clone = Some(widget.clone());
                       node_borrow.init_child(Box::new(move || Pure::new(widget_clone)),widget.clone()).0
                    };
                    let cont = node.clone();
                    let mut prev_node = node.clone();
                    for #variable in #for_expr {
                        let node = prev_node.clone();
                        #content
                        prev_node = node;
                    }
                    {
                        prev_node.as_ref().borrow_mut().get_sibling_mut().take();
                    }
                }
            }
        }
        if let Ok(_) = input.parse::<syn::token::For>() {
            for_recursive(input, (pure_index, init_type))
        } else {
            TokenStream2::new()
        }
    }

    fn get_pure_remove_block(pure_index: u32) -> TokenStream2 {
        quote! {
            let current_index = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                let index = pure.current_index.clone();
                pure.current_index = #pure_index;
                index
            };
            if current_index != #pure_index {
                let node = {
                    node.as_ref().borrow_mut().get_child_mut().take()
                };
            }
        }
    }

    fn parse_condition_block(input: &ParseStream, _pure_index: u32, init_type: &TokenStream2) -> TokenStream2 {
        fn if_recursive(input: ParseStream, pure_index: &mut u32) -> TokenStream2 {
            let comparison_expr = input.parse::<syn::Expr>().unwrap();
            let node = if let Ok(_) = input.parse::<syn::token::If>() {
                if_recursive(input, pure_index)
            } else {
                CParser::custom_parse(input, InitType::PureChild(*pure_index).get_init_type_tuple())
            };
            *pure_index += 1;
            let chain = if let Ok(_) = input.parse::<syn::token::Else>() {
                if let Ok(_) = input.parse::<syn::token::If>() {
                    let tree = if_recursive(input, pure_index);
                    quote!(else #tree)
                } else {
                    let node = CParser::custom_parse(input, InitType::PureChild(*pure_index).get_init_type_tuple());
                    quote!( else { #node } )
                }
            } else {
                let pure_remove_block = CParser::get_pure_remove_block(*pure_index);
                quote! { else {
                    #pure_remove_block
                    let widget = cont.as_ref().borrow().get_widget_as_container();
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let widget_clone = Some(widget.clone());
                    node_borrow.init_child(Box::new(move || Pure::new(widget_clone)),widget.clone());
                }}
            };
            return quote! {
                if #comparison_expr {
                    #node
                } #chain
            };
        }

        if let Ok(_) = input.parse::<syn::token::If>() {
            let mut pure_index = 1;
            let tree = if_recursive(input, &mut pure_index);
            quote!(
               let node = {
                   let widget = cont.as_ref().borrow().get_widget_as_container();
                   let mut node_borrow = node.as_ref().borrow_mut();
                   let widget_clone = Some(widget.clone());
                   node_borrow.#init_type(Box::new(move || Pure::new(widget_clone)),widget.clone()).0
               };
               {
                    let cont = node.clone();
                    let state_borrow = top_state.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                    #tree
               }
            )
        } else {
            TokenStream2::new()
        }
    }

    fn parse_expression(input: ParseStream, pure_index: u32, init_type: TokenStream2) -> TokenStream2 {
        if let Ok(name) = input.parse::<Ident>() {
            let mut static_exprs = vec![];
            let mut dynamic_exprs = vec![];
            //parse property block
            if let Ok(block) = input.parse::<Block>() {
                for x in block.stmts {
                    match x {
                        Stmt::Semi(s, _) => match s {
                            Expr::Assign(e) => {
                                let left = e.left;
                                let right = e.right;
                                match *right {
                                    Expr::Closure(closure) => {
                                        let closure_body = closure.body;
                                        static_exprs.push(quote! {{
                                             let state_clone = Rc::clone(&top_state);
                                             node.widget.#left(move |_| {
                                                 let state = state_clone.clone();
                                                 {
                                                     let mut state_borrow = state.as_ref().borrow_mut();
                                                     let state = state_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
                                                     #closure_body
                                                 }
                                                 Self::render(state.clone());
                                             });
                                        }});
                                    }
                                    Expr::Lit(literal) => {
                                        static_exprs.push(quote! { node.widget.#left(#literal); })
                                    }
                                    _ => dynamic_exprs.push(quote! { node.widget.#left(#right); }),
                                }
                            }
                            _ => panic!("expected an Assignment Expression"),
                        },
                        _ => {
                            panic!("expected an Expression")
                        }
                    }
                }
            }
            let tree = {
                let (pure_state_reference, pure_remove_block) = if pure_index > 0 {
                    (
                        TokenStream2::new(),
                        CParser::get_pure_remove_block(pure_index),
                    )
                } else {
                    (
                        quote! {
                            let mut state_borrow = top_state.as_ref().borrow();
                            let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                        },
                        TokenStream2::new(),
                    )
                };

                quote! {
                    let node = {
                        let (node, is_new) = {
                            { #pure_remove_block }
                            let widget = cont.as_ref().borrow().get_widget_as_container();
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let widget_clone = Some(widget.clone());
                            node_borrow.#init_type(Box::new(move || #name::new(widget_clone)),widget.clone())
                        };
                        {
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let node = node_borrow.as_any_mut().downcast_mut::<#name>().unwrap();
                            if is_new {
                                #(#static_exprs)*
                            }
                            #pure_state_reference
                            #(#dynamic_exprs)*
                        }
                        #name::render(node.clone());
                        node
                    };
                }
            };
            //parse children
            {
                //check for first block
                let tree = match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = CParser::custom_parse(&brackets.content, InitType::Child.get_init_type_tuple());
                        quote! { #tree {  let cont = node.clone(); #content } }
                    }
                    _ => tree,
                };
                //parse ,
                return match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = CParser::custom_parse(&input, InitType::Sibling.get_init_type_tuple());
                        quote! { #tree #content }
                    }
                    _ => tree,
                };
            }
        }
        TokenStream2::new()
    }

    fn custom_parse(input: ParseStream, (pure_index, init_type): (u32, TokenStream2)) -> TokenStream2 {
        let condition_block = CParser::parse_condition_block(&input, pure_index, &init_type);
        let for_parse = CParser::parse_for_block(&input, (pure_index, &init_type));
        let expr = CParser::parse_expression(input, pure_index, init_type);
        quote!(#condition_block #for_parse #expr)
    }
}

impl Parse for CParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let pure_index: u32 = if let Ok(i) = input.parse::<Lit>() {
            if let Lit::Int(i) = i {
                i.base10_parse().unwrap()
            } else {
                panic!("Expected an u32")
            }
        } else {
            0
        };
        let init_type = if input.peek(syn::Ident) && input.peek2(syn::Ident) {
            input.parse::<syn::Ident>()?.into_token_stream()
        } else {
            quote! {init_child}
        };
        Ok(CParser {
            tree: CParser::custom_parse(input, (pure_index, init_type)),
        })
    }
}
