use quote::quote;
use syn::{Block, Error, Expr, Ident, Stmt};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

pub struct Combinations {
    pub name: Ident,
    pub init_type: Ident,
    pub static_exprs: Vec<TokenStream2>,
    pub dynamic_exprs: Vec<TokenStream2>,
    pub is_pure: bool
}

impl Parse for Combinations {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let is_pure = input.parse::<syn::token::Pound>().is_ok();
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
        Ok(Combinations { name, static_exprs, dynamic_exprs, init_type, is_pure })
    }
}