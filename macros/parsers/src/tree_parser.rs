use quote::*;
use syn::*;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseBuffer, ParseStream};

use crate::init_type::InitType;

///
///  parses the given node tree
///
///  Syntax:
///
///  [PureIndex](PureIndex) [InitType](InitType) { [Properties](Properties) } \[ [TreeParser](TreeParser) or [if](IF) or [for](For) ] \]
///
/// # PureIndex (Optional)
///      PureIndex (u32) indicates that the given node is a child of a pure node in an if block.
///      See [Pure](Pure).
///
/// # InitType (Optional)
///      [InitType](InitType) represents the init_ function call on the node. It can of two types init_child and init_sibling
///
/// # Node (Required)
///     Name of the Node
///
/// # Properties
///     Properties of the given node in the syntax { property = }
///
///     Translates to
///     `node.property(value)`
pub struct TreeParser {
    pub tree: TokenStream2,
}

impl TreeParser {
    fn parse_for_block(
        input: ParseStream, (pure_index, init_type): (u32, &TokenStream2),
    ) -> TokenStream2 {
        fn for_recursive(
            input: ParseStream, (pure_index, init_type): (u32, &TokenStream2),
        ) -> TokenStream2 {
            let variable = input.parse::<syn::Ident>().unwrap();
            input.parse::<syn::token::In>().unwrap();
            let for_expr = input.parse::<syn::Expr>().unwrap();
            let content = TreeParser::custom_parse(input, InitType::Sibling.get_init_type_tuple());
            quote! {
                {
                    c!(#pure_index #init_type Pure);
                    let cont = node.clone();
                    let state_borrow = top_state.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                    let node = {
                       let mut node_borrow = node.as_ref().borrow_mut();
                       let weak_cont = Rc::downgrade(&cont);
                       node_borrow.init_child(Box::new(move || Pure::new(weak_cont))).0
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
            let pure_index = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let pure: &mut Pure = node_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                let index = pure.pure_index.clone();
                pure.pure_index = #pure_index;
                index
            };
            if pure_index != #pure_index {
                let node = {
                    node.as_ref().borrow_mut().get_child_mut().take()
                };
            }
        }
    }

    fn parse_condition_block(
        input: &ParseStream, _pure_index: u32, init_type: &TokenStream2,
    ) -> TokenStream2 {
        fn if_recursive(input: ParseStream, pure_index: &mut u32) -> TokenStream2 {
            let comparison_expr = input.parse::<syn::Expr>().unwrap();
            let node = if let Ok(_) = input.parse::<syn::token::If>() {
                if_recursive(input, pure_index)
            } else {
                TreeParser::custom_parse(
                    input,
                    InitType::PureChild(*pure_index).get_init_type_tuple(),
                )
            };
            *pure_index += 1;
            let chain = if let Ok(_) = input.parse::<syn::token::Else>() {
                if let Ok(_) = input.parse::<syn::token::If>() {
                    let tree = if_recursive(input, pure_index);
                    quote!(else #tree)
                } else {
                    let node = TreeParser::custom_parse(
                        input,
                        InitType::PureChild(*pure_index).get_init_type_tuple(),
                    );
                    quote!( else { #node } )
                }
            } else {
                let pure_remove_block = TreeParser::get_pure_remove_block(*pure_index);
                quote! { else {
                    {
                        { #pure_remove_block }
                        let mut node_borrow = node.as_ref().borrow_mut();
                        node_borrow.init_child(Box::new(move || Pure::new(Rc::downgrade(&cont)))).1
                    };
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
                let (node,is_new) = {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.#init_type(Box::new(move || Pure::new(weak_cont)))
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

    fn parse_expression(
        input: ParseStream, pure_index: u32, init_type: TokenStream2,
    ) -> TokenStream2 {
        if let Ok(name) = input.parse::<Ident>() {
            let mut static_props = vec![];
            let mut dynamic_props = vec![];
            //parse properties
            match group::parse_parens(&input) {
                Ok(parens) => {
                    let props_buffer = parens.content;
                    fn parse_props(
                        props_buffer: ParseBuffer, static_exprs: &mut Vec<TokenStream2>,
                        dynamic_exprs: &mut Vec<TokenStream2>,
                    ) {
                        if let Ok(e) = props_buffer.parse::<syn::ExprAssign>() {
                            let left = e.left;
                            let right = e.right;
                            match *right {
                                Expr::Closure(closure) => {
                                    let closure_body = closure.body;
                                    static_exprs.push(quote! {{
                                             let state_clone = Rc::clone(&top_state);
                                             node.#left(move |_| {
                                                 if let ShouldRender::Yes = Self::update(state_clone.clone(),#closure_body) {
                                                    Self::render(state_clone.clone());
                                                 }
                                             });
                                        }});
                                }
                                Expr::Lit(literal) => {
                                    static_exprs.push(quote! { node.#left(#literal); })
                                }
                                _ => dynamic_exprs.push(quote! { node.#left(#right); }),
                            }
                            //parse ,
                            if let Ok(_) = props_buffer.parse::<syn::token::Comma>() {
                                parse_props(props_buffer, static_exprs, dynamic_exprs);
                            }
                        }
                    }
                    parse_props(props_buffer, &mut static_props, &mut dynamic_props);
                }
                _ => (),
            };
            let tree = {
                //if pure_index > 0 then the component is pure
                let (pure_state_reference, pure_remove_block, render_call) = if pure_index > 0 {
                    (
                        TokenStream2::new(),
                        TreeParser::get_pure_remove_block(pure_index),
                        //need not call render on pure function
                        TokenStream2::new(),
                    )
                } else {
                    (
                        //if there are no dynamic props then borrowing state does not make sense
                        if dynamic_props.is_empty() {
                            TokenStream2::new()
                        } else {
                            quote! {
                                let mut state_borrow = top_state.as_ref().borrow();
                                let state = state_borrow.as_any().downcast_ref::<Self>().unwrap();
                            }
                        },
                        TokenStream2::new(),
                        quote!( #name::render(node.clone()); )
                    )
                };

                quote! {
                    let node = {
                        let (node, is_new) = {
                            { #pure_remove_block }
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let weak_cont = Rc::downgrade(&cont);
                            node_borrow.#init_type(Box::new(move || #name::new(weak_cont)))
                        };
                        {
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let node = node_borrow.as_any_mut().downcast_mut::<#name>().unwrap();
                            if is_new {
                                #(#static_props)*
                            }
                            #pure_state_reference
                            #(#dynamic_props)*
                        }
                        #render_call
                        node
                    };
                }
            };
            //parse children
            {
                //check for first block
                let tree = match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = TreeParser::custom_parse(
                            &brackets.content,
                            InitType::Child.get_init_type_tuple(),
                        );
                        quote! { #tree {  let cont = node.clone(); #content } }
                    }
                    _ => tree,
                };
                //parse ,
                return match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = TreeParser::custom_parse(
                            &input,
                            InitType::Sibling.get_init_type_tuple(),
                        );
                        quote! { #tree #content }
                    }
                    _ => tree,
                };
            }
        }
        TokenStream2::new()
    }

    fn custom_parse(
        input: ParseStream, (pure_index, init_type): (u32, TokenStream2),
    ) -> TokenStream2 {
        let condition_block = TreeParser::parse_condition_block(&input, pure_index, &init_type);
        let for_parse = TreeParser::parse_for_block(&input, (pure_index, &init_type));
        let expr = TreeParser::parse_expression(input, pure_index, init_type);
        quote!(#condition_block #for_parse #expr)
    }
}

impl Parse for TreeParser {
    fn parse(input: ParseStream) -> Result<Self> {
        //check for pure_index
        let pure_index: u32 = if let Ok(i) = input.parse::<Lit>() {
            if let Lit::Int(i) = i {
                i.base10_parse().unwrap()
            } else {
                panic!("Expected an u32")
            }
        } else {
            0
        };
        // Both init_type and Node are of type Ident therefore peek and check if init_type is provided or not
        let init_type = if input.peek(syn::Ident) && input.peek2(syn::Ident) {
            input.parse::<syn::Ident>()?.into_token_stream()
        } else {
            quote! {init_child}
        };
        Ok(TreeParser {
            tree: TreeParser::custom_parse(input, (pure_index, init_type)),
        })
    }
}
