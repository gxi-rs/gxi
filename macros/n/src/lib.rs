use proc_macro::TokenStream;

use quote::quote;
use syn::{Block, Error, Expr, Ident, Stmt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

struct Combinations {
    name: Ident,
    init_type: Ident,
    static_exprs: Vec<TokenStream2>,
    dynamic_exprs: Vec<TokenStream2>,
}

impl Parse for Combinations {
    fn parse(input: ParseStream) -> Result<Self, Error> {
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
                                             let container_clone = Rc::clone(&container);
                                             let state_clone = Rc::clone(&state);
                                             node.widget.#left(move |_| {
                                                 let state = state_clone.clone();
                                                 {
                                                     let mut state = state.as_ref().borrow_mut();
                                                     #closure_body
                                                 }
                                                 render(container_clone.clone(), state.clone());
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
        Ok(Combinations { name, static_exprs, dynamic_exprs, init_type })
    }
}

#[proc_macro]
pub fn n(item: TokenStream) -> TokenStream {
    let Combinations { name, static_exprs, dynamic_exprs, init_type } = syn::parse_macro_input!(item as Combinations);

    (quote! {
        let node = {
            let (node, is_new) = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let cont = Rc::clone(&cont);
                node_borrow.#init_type(Box::new(move || #name::new(cont.clone())), true)
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<#name>().unwrap();
                if is_new {
                    #(#static_exprs)*
                }
                let state = state.as_ref().borrow();
                #(#dynamic_exprs)*
            }
            node
        };
    }).into()
}