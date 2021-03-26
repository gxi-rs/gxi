use quote::quote;
use syn::{Block, Error, Expr, Ident, Stmt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

pub struct NParser {
    pub tree: TokenStream2,
}

impl Parse for NParser {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let pure_index: u32 = if let Ok(i) = input.parse::<syn::Lit>() {
            if let syn::Lit::Int(i) = i {
                i.base10_parse().unwrap()
            } else { panic!("Expected an u32") }
        } else { 0 };
        let name = input.parse()?;
        let init_type = input.parse()?;
        let mut static_exprs = vec![];
        let mut dynamic_exprs = vec![];
        {
            let block: Block = input.parse()?;
            for x in block.stmts {
                match x {
                    Stmt::Semi(s, _) => {
                        match s {
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
                                    Expr::Lit(literal) => static_exprs.push(quote! { node.widget.#left(#literal); }),
                                    _ => dynamic_exprs.push(quote! { node.widget.#left(#right); })
                                }
                            }
                            _ => panic!("expected an Assignment Expression")
                        }
                    }
                    _ => { panic!("expected an Expression") }
                }
            }
        }
        {
            let (pure_state_reference, pure_remove_block) = if pure_index > 0 {
                (TokenStream2::new(), quote! {
                    let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                    if pure.current_index != #pure_index {
                        if let Some(child) = pure.child.as_ref() {
                            pure.get_widget_as_container().remove(&child.as_ref().borrow().get_widget());
                            pure.child = None;
                        }
                        pure.current_index = #pure_index;
                    }
                })
            } else {
                (quote! {
                    let mut state_borrow = top_state.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                }, TokenStream2::new())
            };

            Ok(NParser {
                tree: (quote! {
                    let node = {
                    let (node, is_new) = {
                        let widget = Some(cont.as_ref().borrow().get_widget_as_container());
                        let mut node_borrow = node.as_ref().borrow_mut();
                        { #pure_remove_block }
                        let cont = Rc::clone(&cont);
                        node_borrow.#init_type(Box::new(move || #name::new(cont.clone(),widget)), if let NodeType::Widget = #name::get_type() { true } else { false })
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
                })
            })
        }
    }
}