use quote::*;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::*;

use crate::init_type::InitType;

///
///  parses the given node tree
///
///  Syntax:
///
///  [PureIndex](PureIndex) [InitType](InitType) { [Properties](Properties) } \[ [TreeParser](TreeParser) or [if](IF) or [for](For) ] \]
///
/// # PureIndex (Optional)
///      PureIndex (u32) indicates that the given node is a.rs child of a.rs pure node in an if block.
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
    fn parse_for_block(input: ParseStream, init_type: &InitType) -> TokenStream2 {
        fn for_recursive(input: ParseStream, init_type: &InitType) -> TokenStream2 {
            let variable = input.parse::<syn::Ident>().unwrap();
            input.parse::<syn::token::In>().unwrap();
            let for_expr = input.parse::<syn::Expr>().unwrap();
            let content = TreeParser::custom_parse(input, InitType::Sibling(0));
            let (pure_index, init_type) = init_type.get_init_type_tuple();
            quote! {
                let node = {
                    c!(#pure_index #init_type Pure);
                    {
                        let cont = node.clone();
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
                    node
                };
            }
        }
        if let Ok(_) = input.parse::<syn::token::For>() {
            for_recursive(input, init_type)
        } else {
            TokenStream2::new()
        }
    }

    fn get_pure_remove_block(pure_index: u32) -> TokenStream2 {
        quote! {{
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
        }}
    }

    fn parse_condition_block(input: &ParseStream, init_type: &InitType) -> TokenStream2 {
        fn if_recursive(input: ParseStream, pure_index: &mut u32) -> TokenStream2 {
            let comparison_expr = input.parse::<syn::Expr>().unwrap();
            let node = if let Ok(_) = input.parse::<syn::token::If>() {
                if_recursive(input, pure_index)
            } else {
                TreeParser::custom_parse(input, InitType::Child(*pure_index))
            };
            *pure_index += 1;
            let chain = if let Ok(_) = input.parse::<syn::token::Else>() {
                if let Ok(_) = input.parse::<syn::token::If>() {
                    let tree = if_recursive(input, pure_index);
                    quote!(else #tree)
                } else {
                    let node = TreeParser::custom_parse(input, InitType::Child(*pure_index));
                    quote!( else { #node } )
                }
            } else {
                let pure_remove_block = TreeParser::get_pure_remove_block(*pure_index);
                quote! { else {
                    {
                        #pure_remove_block
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let weak_cont = Rc::downgrade(&cont);
                        node_borrow.init_child(Box::new(move || Pure::new(weak_cont))).1
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
            let init_type = init_type.get_init_type_tuple().1;
            quote!(
                let (node,is_new) = {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.#init_type(Box::new(move || Pure::new(weak_cont)))
                };
                {
                     let cont = node.clone();
                     #tree
                }
            )
        } else {
            TokenStream2::new()
        }
    }

    fn parse_expression(input: ParseStream, init_type: &InitType) -> TokenStream2 {
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
                                        let state_clone = Rc::clone(&this);
                                        node.#left(move | | Self::update(state_clone.clone(),#closure_body) );
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
            let (tree, render_call) = {
                let pre_init = match init_type {
                    InitType::Child(_) => {
                        quote! {
                            let node = cont.clone();
                        }
                    }
                    InitType::Sibling(_) => TokenStream2::new(),
                };
                let (pure_index, init_type) = init_type.get_init_type_tuple();
                //if pure_index > 0 then the component is pure
                let (pure_remove_block, render_call) = if pure_index > 0 {
                    (
                        TreeParser::get_pure_remove_block(pure_index),
                        //need not call render on pure function
                        TokenStream2::new(),
                    )
                } else {
                    (TokenStream2::new(), quote!( #name::render(node.clone()); ))
                };

                (
                    quote! {
                        let node = {
                            let (node, is_new) = {
                                #pure_remove_block
                                #pre_init
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
                                #(#dynamic_props)*
                            }
                            #render_call
                            node
                        };
                    },
                    render_call,
                )
            };
            //parse children
            {
                //check for first block
                let children = match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content =
                            TreeParser::custom_parse(&brackets.content, InitType::Child(0));
                        if content.is_empty() {
                            TokenStream2::new()
                        } else {
                            quote! {
                                {
                                    let cont = {
                                        let node_borrow = node.as_ref().borrow();
                                        node_borrow.get_self_substitute()
                                    };
                                    #content
                                }
                                #render_call
                            }
                        }
                    }
                    _ => TokenStream2::new(),
                };
                let tree = quote! { #tree #children };
                //recursive function therefore drop whatever memory possible
                drop(render_call);
                drop(children);
                //parse ,
                return match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = TreeParser::custom_parse(&input, InitType::Sibling(0));
                        quote! { #tree #content }
                    }
                    _ => tree,
                };
            }
        }
        TokenStream2::new()
    }

    fn parse_child_injection(input: ParseStream, init_type: &InitType) -> TokenStream2 {
        if let Ok(_) = input.parse::<syn::token::Pound>() {
            let ident = input.parse::<syn::Ident>().unwrap();
            let (_, init_type) = init_type.get_init_type_tuple();
            match &ident.to_string()[..] {
                "children" => {
                    quote! {
                        let node = {
                            let node = {
                                let mut node_borrow = node.as_ref().borrow_mut();
                                let weak_cont = Rc::downgrade(&cont);
                                node_borrow.#init_type(Box::new(move || Pure::new(weak_cont))).0
                            };
                            {
                                let mut this_borrow = this.as_ref().borrow_mut();
                                this_borrow.set_self_substitute(node.clone());
                            }
                            node
                        };
                    }
                }
                _ => {
                    panic!("Expected an attribute here. Eg [children]")
                }
            }
        } else {
            TokenStream2::new()
        }
    }
    fn parse_block(input: ParseStream) -> TokenStream2 {
        if let Ok(b) = input.parse::<syn::Block>() {
            quote! {{ #b }}
        } else {
            TokenStream2::new()
        }
    }

    fn custom_parse(input: ParseStream, init_type: InitType) -> TokenStream2 {
        {
            let condition_block = TreeParser::parse_condition_block(&input, &init_type);
            if !condition_block.is_empty() {
                return condition_block;
            }
        }
        {
            let for_parse = TreeParser::parse_for_block(&input, &init_type);
            if !for_parse.is_empty() {
                return for_parse;
            }
        }
        {
            let child = TreeParser::parse_child_injection(&input, &init_type);
            if !child.is_empty() {
                return child;
            }
        }
        {
            let block = TreeParser::parse_block(&input);
            if !block.is_empty() {
                return block;
            }
        }
        {
            let expr = TreeParser::parse_expression(input, &init_type);
            if !expr.is_empty() {
                return expr;
            }
        }
        return TokenStream2::new();
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
            match &input.parse::<syn::Ident>()?.to_string()[..] {
                "init_child" => InitType::Child(pure_index),
                "init_sibling" => InitType::Sibling(pure_index),
                _ => {
                    panic!("Expected init_type as init_child or init_sibling ");
                }
            }
        } else {
            InitType::Child(pure_index)
        };

        Ok(TreeParser {
            tree: TreeParser::custom_parse(input, init_type),
        })
    }
}
