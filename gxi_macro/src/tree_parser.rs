use quote::{quote};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::Result;

use crate::InitType;

/// Parser for the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html).
pub struct TreeParser(pub TokenStream2);

impl Parse for TreeParser {
    /// TODO: update doc
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if input.is_empty() {
            TreeParser(TokenStream2::new())
        } else {
            // default init type is child
            TreeParser(TreeParser::custom_parse(input, InitType::Child, false)?)
        })
    }
}

impl TreeParser {
    /// Parses the `for` block as defined in the [Looping Section][../gxi_c_macro/macro.gxi_c_macro.html#Looping] of the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html).
    fn parse_for_block(input: ParseStream, init_type: &InitType) -> Result<TokenStream2> {
        fn for_recursive(input: ParseStream, init_type: &InitType) -> Result<TokenStream2> {
            let variable = input.parse::<syn::Ident>()?;
            input.parse::<syn::token::In>()?;
            let for_expr = input.parse::<syn::Expr>()?;
            let content = TreeParser::custom_parse(input, InitType::Sibling, false)?;
            let pure = {
                let pure = quote!(#init_type Pure);
                let tree_parser: TreeParser = syn::parse2(pure)?;
                tree_parser.0
            };
            Ok(quote! {
                let node = {
                    #pure
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
            })
        }
        if let Ok(_) = input.parse::<syn::token::For>() {
            for_recursive(input, init_type)
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// generates the block to correctly drop `Pure` component without violating mutable rules.
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

    /// Parses the `if` block as defined in the [Conditional Rendering Section][../gxi_c_macro/macro.gxi_c_macro.html#Conditional-Rendering] of the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html).
    ///
    /// *internals*
    ///
    /// Each `if` block is wrapped inside a `Pure` component. The Pure node has a `pure_index` of 0. Each wing of the if block is given a `pure_index`.
    /// When a `branch` of the if block is true, its `pure_index` is compared to the `pure_index` of the parent. If it is true then it means that
    /// during the previous render this `branch` was true which means that the element in this `branch` is already initialized. Therefore it is not required
    /// to be initialized. If the pure_index of parent and the if branch is not the same then it means that during the previous render `branch` was not
    /// initialized therefore it needs to be initialized now.
    ///
    /// If an else branch is not provided then an else branch with a Pure node is appended.
    ///
    fn parse_condition_block(input: &ParseStream, init_type: &InitType) -> Result<TokenStream2> {
        // check for if
        if input.parse::<syn::token::If>().is_ok() {
            let mut pure_index = 0;
            let mut if_logic = input.parse::<syn::Expr>()?;
            // chain starts with if block
            let mut chain = quote! { if #if_logic };
            loop {
                pure_index += 1;
                let parsed_block = {
                    let block = syn::group::parse_braces(&input)?.content;
                    TreeParser::parse(&block)?.0
                };
                // concatenate
                {
                    let pure_remove_block = TreeParser::get_pure_remove_block(pure_index);
                    chain = quote! { #chain {
                        #pure_remove_block
                        #parsed_block
                    }};
                }
                // check for else
                if input.is_empty() {
                    break;
                } else if input.parse::<syn::token::Else>().is_ok() {
                    chain = quote! { #chain else };
                    // check for if, i.e else if block
                    if input.parse::<syn::token::If>().is_ok() {
                        if_logic = input.parse::<syn::Expr>()?;
                        chain = quote! { #chain if #if_logic };
                    }
                } else {
                    // if no else block then add a custom one which when executes destroys any existing child
                    let pure_remove_block = TreeParser::get_pure_remove_block(pure_index + 1);
                    chain = quote! { #chain else #pure_remove_block };
                    break;
                }
            }

            Ok(quote! {
                let node = {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let weak_cont = Rc::downgrade(&cont);
                    node_borrow.#init_type(Box::new(move || Pure::new(weak_cont))).0
                };
                {
                    let cont = node.clone();
                    #chain
                }
            })
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// Parses the Component with its properties and its children recursively from the syntax defined by the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html)
    fn parse_component(input: &ParseStream, init_type: &InitType) -> Result<TokenStream2> {
        if let Ok(name) = input.parse::<syn::Ident>() {
            let mut static_props = vec![];
            let mut dynamic_props = vec![];
            //parse properties enclosed in parenthesis
            if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(&input) {
                // loop till every thing inside parenthesis is parsed
                loop {
                    if let Ok(syn::ExprAssign { left, right, .. }) =
                    content.parse::<syn::ExprAssign>()
                    {
                        // push closure and literals to static_props and others to dynamic_props
                        match *right {
                            syn::Expr::Closure(closure) => {
                                let closure_body = closure.body;
                                static_props.push(quote! {{
                                        let state_clone = Rc::clone(&this);
                                        node.#left(move | | Self::update(state_clone.clone(),#closure_body) );
                                    }});
                            }
                            syn::Expr::Lit(literal) => {
                                static_props.push(quote! { node.#left(#literal); })
                            }
                            _ => dynamic_props.push(quote! { node.#left(#right); }),
                        }
                        // if stream is empty then break
                        if content.is_empty() {
                            break;
                        } else {
                            // else expect a comma
                            content.parse::<syn::token::Comma>()?;
                        }
                    } else {
                        break;
                    }
                }
            }
            // create node initializer methods
            let component = {
                // block which calls prop setter functions of the node
                let prop_setter_block = {
                    // if there are no props then return an empty TokenStream
                    if dynamic_props.is_empty() && static_props.is_empty() {
                        TokenStream2::new()
                    } else {
                        let mut block = quote! {
                            let mut node_borrow = node.as_ref().borrow_mut();
                            let node = node_borrow.as_any_mut().downcast_mut::<#name>().unwrap();
                        };
                        if !static_props.is_empty() {
                            block = quote! {
                                #block
                                if is_new {
                                    #(#static_props)*
                                }
                            };
                        }
                        if !dynamic_props.is_empty() {
                            block = quote! {
                                #block
                                #(#dynamic_props)*
                            };
                        }
                        quote! {{
                            #block
                        }}
                    }
                };
                // if init_type is child then add to cont else add to node
                let node_rename = if let InitType::Child = init_type {
                    quote! { cont }
                } else {
                    quote! { node }
                };

                quote! {
                    let node = {
                        let (node, is_new) = {
                            let mut node_borrow = #node_rename.as_ref().borrow_mut();
                            let weak_cont = Rc::downgrade(&cont);
                            node_borrow.#init_type(Box::new(move || #name::new(weak_cont)))
                        };
                        #prop_setter_block
                        #name::render(node.clone());
                        node
                    };
                }
            };

            // parse children
            let children = if let syn::__private::Ok(syn::group::Brackets { content, .. }) =
            syn::group::parse_brackets(&input)
            {
                // if content is empty don't parse it
                if content.is_empty() {
                    TokenStream2::new()
                } else {
                    // parse content with init_child and pure_index 0
                    let content = TreeParser::custom_parse(&content, InitType::Child, true)?;
                    // set parent and concatenate the parsed content
                    quote! {
                        let cont = {
                            let node_borrow = node.as_ref().borrow();
                            node_borrow.get_self_substitute()
                        };
                        #content
                    }
                }
            } else {
                TokenStream2::new()
            };

            // concatenate every thing
            Ok(quote! {
                #component
                {
                    #children
                }
                #name::render(node.clone());
            })
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// Parses the `#children` statement as defined in the [#children statement section][../gxi_c_macro/macro.gxi_c_macro.html#children-statement] of the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html).
    fn parse_child_injection(input: ParseStream, init_type: &InitType) -> Result<TokenStream2> {
        if let Ok(_) = input.parse::<syn::token::Pound>() {
            let ident = input.parse::<syn::Ident>()?;
            return match &ident.to_string()[..] {
                "children" => Ok(quote! {
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
                }),
                _ => Err(syn::Error::new(
                    ident.span().unwrap().into(),
                    "Expected children here",
                )),
            };
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// anything inside a {} is copied and executed on every render call
    fn parse_execution_block(input: ParseStream) -> Result<TokenStream2> {
        if let Ok(b) = input.parse::<syn::Block>() {
            Ok(quote! {{ #b }})
        } else {
            Ok(TokenStream2::new())
        }
    }

    fn custom_parse(input: ParseStream, mut init_type: InitType, can_have_more_than_one_root_node: bool) -> Result<TokenStream2> {
        let mut tree = TokenStream2::new();
        let mut has_one_root_component = false;
        loop {
            // check if input is empty
            if input.is_empty() {
                return Ok(tree);
            }
            // match and parse
            let parsed = {
                let component_block = TreeParser::parse_child_injection(&input, &init_type)?;
                if component_block.is_empty() {
                    let for_parse = TreeParser::parse_for_block(&input, &init_type)?;
                    if for_parse.is_empty() {
                        let conditional_block = TreeParser::parse_condition_block(&input, &init_type)?;
                        if conditional_block.is_empty() {
                            let execution_block = TreeParser::parse_execution_block(&input)?;
                            if execution_block.is_empty() {
                                // if the tree already has a root component and it can't have more than one then throw error
                                if !can_have_more_than_one_root_node && has_one_root_component {
                                    return Err(syn::Error::new(
                                        input.span().unwrap().into(),
                                        "didn't expect this here. Help: You can't have more than one node here.",
                                    ));
                                }
                                let component = TreeParser::parse_component(&input, &init_type)?;
                                // there can only be one root component
                                has_one_root_component = true;
                                // there can only be one child, therefore after parsing a component
                                // it is guaranteed that the next component will be sibling
                                init_type = InitType::Sibling;
                                component
                            } else {
                                execution_block
                            }
                        } else {
                            conditional_block
                        }
                    } else {
                        for_parse
                    }
                } else {
                    component_block
                }
            };

            if parsed.is_empty() {
                return Err(syn::Error::new(input.span().unwrap().into(), "didn't expect this here"));
            } else {
                tree = quote! { #tree #parsed };
                // there has to be a comma if input is not empty and the previous parse was successful
                if !input.is_empty() && input.parse::<syn::token::Comma>().is_err() {
                    return Err(syn::Error::new(
                        input.span().unwrap().into(),
                        ", expected here",
                    ));
                }
            }
        }
    }
}
