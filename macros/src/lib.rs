use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{Block, Error, Expr, Ident, Stmt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

struct Combinations {
    name: Ident,
    static_exprs: Vec<TokenStream2>,
    dynamic_exprs: Vec<TokenStream2>,
}

impl Parse for Combinations {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let name = input.parse()?;
        let mut static_exprs = vec![];
        let mut dynamic_exprs = vec![];
        {
            let block: Block = input.parse()?;
            for x in block.stmts {
                println!("{}", x.to_token_stream().to_string());
                match x {
                    Stmt::Semi(s, _) => {
                        match s {
                            Expr::Assign(e) => {
                                let left = e.left;
                                let right = e.right;
                                match *right {
                                    Expr::Lit(literal) => static_exprs.push(quote! { node.widget.#left(#literal); }),
                                    Expr::Closure(closure) => {
                                        let closure_body = closure.body;
                                        static_exprs.push((quote! {{
                                             let container_clone = Rc::clone(&container);
                                             node.widget.#left(move | | {
                                                 let state = state.clone();
                                                 {
                                                     let mut state = state.as_ref().borrow_mut();
                                                     #closure_body
                                                 }
                                                 render(container_clone.clone(), state.clone());
                                             });
                                        }}));
                                    }
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
        Ok(Combinations { name, static_exprs, dynamic_exprs })
    }
}

#[proc_macro]
pub fn proc_node(item: TokenStream) -> TokenStream {
    let Combinations { name, static_exprs, dynamic_exprs } = syn::parse_macro_input!(item as Combinations);

    (quote! {
        let node = {
            let (node, is_new) = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = Rc::clone(&node);
                node_borrow.init_child(Box::new(move || #name::new(node.clone())), true)
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<#name>().unwrap();
                let state = state.as_ref().borrow();
                if is_new {
                    #(#static_exprs)*
                }
                #(#dynamic_exprs)*
            }

            node
        };
    }).into()
}
/*
#[proc_macro]
pub fn proc_node(item: TokenStream) -> TokenStream {
    let name= syn::parse_macro_input!(item as syn::Ident);
    let name = &name;
    let result = quote! {
        {
            let (node, _is_new) = {
                let mut node_borrow = $prev_node.as_ref().borrow_mut();
                let container = Rc::clone(&$prev_node);
                node_borrow.$type(Box::new(move || $Node::new(container.clone())), true)
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<$Node>().unwrap();
                let state = $state.as_ref().borrow();
                $(
                    node.widget.$f(state.$v.to_string().as_str());
                )*
            }
            node
        }
    };
    result.into()
}
*/