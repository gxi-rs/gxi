use quote::*;
use syn::*;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

use crate::init_type::InitType;

pub struct CParser {
    pub tree: TokenStream2,
}

impl CParser {
    fn parse_execution_block(input: &ParseStream, init_type: &InitType) -> TokenStream2 {
        if let Ok(block) = input.parse::<Block>() {
            let content = CParser::custom_parse(input, InitType::Sibling);
            let init_type = init_type.get_init_quote().1;
            return quote! {
                let node = {
                    let node = {
                        let widget = Some(cont.as_ref().borrow().get_widget_as_container());
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let cont = Rc::clone(&cont);
                        node_borrow.#init_type(Box::new(move || Pure::new(cont.clone(),widget)), false).0
                    };
                    let cont = node.clone();
                    let node = {
                        let widget = Some(cont.as_ref().borrow().get_widget_as_container());
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let cont = Rc::clone(&cont);
                        node_borrow.init_child(Box::new(move || Pure::new(cont.clone(), widget)), false).0
                    };
                    let mut state_borrow = top_state.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                    #block
                    node
                };
                #content
            };
        };
        return TokenStream2::new();
    }

    fn parse_condition_block(input: &ParseStream, init_type: &InitType) -> TokenStream2 {
        let init_type = init_type.get_init_quote().1;
        /*
            if comp
                node
            else if
        */
        fn iff_recursive(input: ParseStream, pure_index: u32) -> TokenStream2 {
            let comparison_expr = input.parse::<syn::Expr>().unwrap();
            let node = CParser::custom_parse(input, InitType::Pure(pure_index));
            let pure_index = pure_index + 1;
            let chain = if let Ok(_) = input.parse::<syn::token::Else>() {
                if let Ok(_) = input.parse::<syn::token::If>() {
                    let tree = iff_recursive(input, pure_index);
                    quote!(else if #tree)
                } else {
                    let node =  CParser::custom_parse(input, InitType::Pure(pure_index));
                    quote!( else { #node } )
                }
            } else {
                quote! { else { c!(#pure_index Pure); } }
            };
            return quote! {
                if #comparison_expr {
                    #node
                } #chain
            };
        }

        if let Ok(_) = input.parse::<syn::token::If>() {
            let tree = iff_recursive(input, 1);
            quote!(
               let node = {
                   let widget = Some(cont.as_ref().borrow().get_widget_as_container());
                   let mut node_borrow = node.as_ref().borrow_mut();
                   let cont = Rc::clone(&cont);
                   node_borrow.#init_type(Box::new(move || Pure::new(cont.clone(),widget)), false).0
               };
               {
                    let cont = node.clone();
                    let mut state_borrow = top_state.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                    #tree
               }
            )
        } else {
            TokenStream2::new()
        }
    }

    fn parse_expression(input: ParseStream, init_type: InitType) -> TokenStream2 {
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
                let (pure_index, init_type) = init_type.get_init_quote();
                let (pure_state_reference, pure_remove_block) = if pure_index > 0 {
                    (
                        TokenStream2::new(),
                        quote! {
                            let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                            if pure.current_index != #pure_index {
                                if let Some(child) = pure.child.as_ref() {
                                    pure.get_widget_as_container().remove(&child.as_ref().borrow().get_widget());
                                    pure.child = None;
                                }
                                pure.current_index = #pure_index;
                            }
                        },
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
                            let widget = Some(cont.as_ref().borrow().get_widget_as_container());
                            let mut node_borrow = node.as_ref().borrow_mut();
                            { #pure_remove_block }
                            let cont = Rc::clone(&cont);
                            node_borrow.#init_type(Box::new(move || #name::new(cont.clone(),widget)), #name::get_type().should_add_widget())
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
                        let content = CParser::custom_parse(&brackets.content, InitType::Child);
                        quote! { #tree {  let cont = node.clone(); #content } }
                    }
                    _ => tree,
                };
                //parse ,
                return match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = CParser::custom_parse(&input, InitType::Sibling);
                        quote! { #tree #content }
                    }
                    _ => tree,
                };
            }
        }
        TokenStream2::new()
    }

    fn custom_parse(input: ParseStream, init_type: InitType) -> TokenStream2 {
        let condition_block = CParser::parse_condition_block(&input, &init_type);
        let expr = CParser::parse_expression(input, init_type);
        quote!(#condition_block #expr)
    }
}

impl Parse for CParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let init_type = if let Ok(i) = input.parse::<syn::Lit>() {
            if let Lit::Int(i) = i {
                let i = i.base10_parse().unwrap();
                if i > 0 {
                    InitType::Pure(i)
                } else {
                    panic!("Expected an u32 greater than 1")
                }
            } else {
                panic!("Expected an u32")
            }
        } else {
            InitType::Child
        };
        Ok(CParser {
            tree: CParser::custom_parse(input, init_type),
        })
    }
}
