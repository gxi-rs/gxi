use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, Result};

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
            TreeParser(TreeParser::custom_parse(
                input,
                InitType::Child,
                false,
                true,
            )?)
        })
    }
}

/// check if the data source has linear data
/// i.e it can't have any unexpected values in between
/// eg, 0..n can't have any new values inserted at an index say 2
/// variable a can have an un-ordered changing iter
fn is_data_source_linear(data_source: &Expr) -> bool {
    match data_source {
        Expr::Array(_) | Expr::Range(_) | Expr::Repeat(_) => true,
        Expr::Paren(paren) => is_data_source_linear(&paren.expr),
        _ => false,
    }
}

impl TreeParser {
    /// Parses the `for` block as defined in the [Looping Section][../gxi_c_macro/macro.gxi_c_macro.html#Looping] of the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html).
    fn parse_for_block(
        input: ParseStream,
        init_type: &InitType,
        is_first_component: bool,
    ) -> Result<TokenStream2> {
        if input.parse::<syn::token::For>().is_ok() {
            // parse : for loop_variable in loop_data_source { }
            let loop_variable = input.parse::<syn::Expr>()?;
            input.parse::<syn::token::In>()?;
            let loop_data_source = input.parse::<syn::Expr>()?;

            let is_data_source_linear = is_data_source_linear(&loop_data_source);

            return if is_data_source_linear {
                // parse the block with InitType::Sibling
                let parsed_loop_block = {
                    let block_content = syn::group::parse_braces(&input)?.content;
                    Self::custom_parse(&block_content, InitType::Sibling, true, false)?
                };

                Ok(quote! {
                    // parent node which will hold all the nodes from the for loop
                    let (node, ..) = init_member(node.clone(), #init_type, |this| Pure::new(this), #is_first_component);
                    {
                        // this node will act as the child of pure block
                        // because there can be only one child but many siblings
                        // component inside the loop will be the sibling of this pure node
                        let (node, ..) = init_member(node.clone(), InitType::Child, |this| Pure::new(this), false);
                        // prev_sibling
                        let mut prev_sibling = node.clone();
                        for #loop_variable in #loop_data_source {
                            let node = prev_sibling.clone();
                            #parsed_loop_block
                            prev_sibling = node;
                        }
                        // drop any left in the tree
                        // because the for loop may run a little less than the previous run
                        *prev_sibling.as_ref().borrow_mut().as_node_mut().get_sibling_mut() = None;
                    }
                })
            } else {
                // parse where clause
                // where var:type
                if input.parse::<syn::token::Where>().is_err() {
                    return Err(syn::Error::new(
                        input.span().unwrap().into(),
                        // TODO: add docs link
                        format!(
                            r#"expected a where clause here.
Since {loop_data_source} can change in order, you have to provide a key for the pattern.
Eg. for {loop_variable} in {loop_data_source} where {loop_variable}:String"#,
                            loop_data_source = loop_data_source.to_token_stream().to_string(),
                            loop_variable = loop_variable.to_token_stream().to_string()
                        ),
                    ));
                }

                let key = input.parse::<syn::Ident>()?;
                input.parse::<syn::Token![:]>()?;
                let key_type = input.parse::<syn::Type>()?;

                // parse the block with InitType::Sibling
                let parsed_loop_block = {
                    let block_content = syn::group::parse_braces(&input)?.content;
                    Self::custom_parse(&block_content, InitType::Sibling, true, false)?
                };

                Ok(quote! {
                    let node = {
                        // parent node which will hold all the nodes from the for loop
                        let (__node, ..) = init_member(node.clone(), #init_type, |this| ForWrapper::<#key_type>::new(this), #is_first_component);

                        for #loop_variable in #loop_data_source {

                            // TODO: add a where clause for key name
                            let (node, is_new) = ForWrapper::<#key_type>::init_child(__node.clone(), #key.clone());
                            #parsed_loop_block
                        }

                        ForWrapper::<#key_type>::clear_unused(__node.clone());

                        __node
                    };
                })
            };
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// generates the block to correctly drop `Pure` component without violating mutable rules.
    fn get_pure_remove_block(pure_index: u32) -> TokenStream2 {
        quote! {{
            let mut node_borrow = node.as_ref().borrow_mut();
            let pure = node_borrow.as_node_mut().as_any_mut().downcast_mut::<Pure>().unwrap();
            if pure.pure_index != #pure_index {
                pure.pure_index = #pure_index;
                pure.child = None;
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
    fn parse_condition_block(
        input: &ParseStream,
        init_type: &InitType,
        is_first_component: bool,
    ) -> Result<TokenStream2> {
        // check for if
        if input.parse::<syn::token::If>().is_ok() {
            let mut pure_index = 0;
            let mut if_logic: TokenStream2 = input.parse::<syn::Expr>()?.to_token_stream();
            // chain starts with if block
            let mut chain = quote! { if #if_logic };
            loop {
                pure_index += 1;
                // concatenate
                {
                    let parsed_block = {
                        let block = syn::group::parse_braces(&input)?.content;
                        TreeParser::custom_parse(&block, InitType::Child, true, false)?
                    };
                    let pure_remove_block = TreeParser::get_pure_remove_block(pure_index);
                    chain = quote! { #chain {
                        #pure_remove_block
                        #parsed_block
                    }};
                }
                // if there is no if_logic then if expression has ended
                if if_logic.is_empty() {
                    break;
                } else if input.parse::<syn::token::Else>().is_ok() {
                    chain = quote! { #chain else };
                    // reset if logic
                    if_logic = TokenStream2::new();
                    // check for nested if
                    if input.parse::<syn::token::If>().is_ok() {
                        // parse if_logic of nested if
                        if_logic = input.parse::<syn::Expr>()?.to_token_stream();
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
                let (node, ..) = init_member(node.clone(), #init_type, |this| Pure::new(this), #is_first_component);
                { #chain }
            })
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// Parses the Component with its properties and its children recursively from the syntax defined by the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html)
    fn parse_component(
        input: &ParseStream,
        init_type: &InitType,
        is_first_component: bool,
    ) -> Result<TokenStream2> {
        if let Ok(path_expr) = input.parse::<syn::Path>() {
            // if last path segment is in lower case them it as a function call on Self
            let (name, init_call) = {
                let last_segment = path_expr.segments.last().unwrap();
                let last_segment_token_stream = last_segment.to_token_stream();
                // if it's first char is in lower case then it is a function call
                if last_segment_token_stream
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .is_lowercase()
                {
                    // there must be least of 2 segments
                    if path_expr.segments.len() < 2 {
                        return Err(syn::Error::new(
                            path_expr.span().unwrap().into(),
                            "There must be at least 2 segments in this path. Eg. Comp::new()",
                        ));
                    }
                    // the segment before the last segment is the required name of the node
                    let name = &path_expr.segments[path_expr.segments.len() - 2];
                    let syn::group::Parens { content, .. } = syn::group::parse_parens(&input)?;
                    let mut params = vec![];
                    loop {
                        params.push(content.parse::<syn::Expr>()?);
                        // if stream is empty then break
                        if content.is_empty() {
                            break;
                        } else {
                            // else expect a comma
                            content.parse::<syn::token::Comma>()?;
                        }
                    }
                    (
                        name.to_token_stream(),
                        quote! {#last_segment(parent, #(#params),*)},
                    )
                } else {
                    (last_segment_token_stream, quote! {new(parent)})
                }
            };
            let mut static_props = vec![];
            let mut dynamic_props = vec![];
            //parse properties enclosed in parenthesis
            if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(&input) {
                // loop till every thing inside parenthesis is parsed
                while let Ok(syn::ExprAssign { left, right, .. }) =
                    content.parse::<syn::ExprAssign>()
                {
                    // push closure and literals to static_props and others to dynamic_props
                    match *right {
                        syn::Expr::Closure(closure) => {
                            let closure_body = closure.body;
                            let closure_args = closure.inputs;
                            static_props.push(quote! {{
                                        let state_clone = Rc::clone(&this);
                                        node.#left(move |#closure_args| Self::update(state_clone.clone(),#closure_body) );
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
                            let mut node = node.as_ref().borrow_mut();
                            let node = node.as_node_mut().as_any_mut().downcast_mut::<#name>().unwrap();
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

                // if init_type is child then get self_substitute
                quote! {
                    let node = {
                        let (node, is_new) = init_member(node.clone(), #init_type, |parent| #name::#init_call, #is_first_component);
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
                    TreeParser::custom_parse(&content, InitType::Child, true, false)?
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
                //#name::render(node.clone());
            })
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// Parses the `#children` statement as defined in the [#children statement section][../gxi_c_macro/macro.gxi_c_macro.html#children-statement] of the [gxi_c_macro macro](../gxi_c_macro/macro.gxi_c_macro.html).
    fn parse_child_injection(
        input: ParseStream,
        init_type: &InitType,
        is_first_component: bool,
    ) -> Result<TokenStream2> {
        if input.parse::<syn::token::Pound>().is_ok() {
            let ident = input.parse::<syn::Ident>()?;
            match &ident.to_string()[..] {
                "children" => Ok(quote! {
                    let node = {
                        let (node, ..) = init_member(node.clone(), #init_type, |this| Pure::new(this), #is_first_component);
                        {
                            let mut this_borrow = this.as_ref().borrow_mut();
                            match this_borrow.deref_mut() {
                                GxiNodeType::Component(t) => *t.get_self_substitute_mut() = Some(Rc::downgrade(&node)),
                                _ => unreachable!(),
                            }
                        }
                        node
                    };
                }),
                _ => Err(syn::Error::new(
                    ident.span().unwrap().into(),
                    "Expected children here",
                )),
            }
        } else {
            Ok(TokenStream2::new())
        }
    }

    /// anything inside a {} is copied and executed on every render call
    fn parse_execution_block(input: ParseStream) -> Result<TokenStream2> {
        if let Ok(b) = input.parse::<syn::Block>() {
            let stmts = &b.stmts;
            Ok(quote! { #(#stmts)* })
        } else {
            Ok(TokenStream2::new())
        }
    }

    fn custom_parse(
        input: ParseStream,
        mut init_type: InitType,
        can_have_more_than_one_root_node: bool,
        mut is_first_component: bool,
    ) -> Result<TokenStream2> {
        let mut tree = TokenStream2::new();
        let mut has_one_root_component = false;
        loop {
            // check if input is empty
            if input.is_empty() {
                return Ok(tree);
            }
            // match and parse
            let parsed = {
                let execution_block = TreeParser::parse_execution_block(&input)?;
                if execution_block.is_empty() {
                    // for, conditional and execution_block functions produce a component
                    // if the tree already has a root component and it can't have more than one then throw error
                    if !can_have_more_than_one_root_node && has_one_root_component {
                        return Err(syn::Error::new(
                            input.span().unwrap().into(),
                            "didn't expect this here. Help: You can't have more than one node here.",
                        ));
                    }
                    let component_block =
                        TreeParser::parse_component(&input, &init_type, is_first_component)?;
                    let parsed = if component_block.is_empty() {
                        let conditional_block = TreeParser::parse_condition_block(
                            &input,
                            &init_type,
                            is_first_component,
                        )?;
                        if conditional_block.is_empty() {
                            let child_injection = TreeParser::parse_child_injection(
                                &input,
                                &init_type,
                                is_first_component,
                            )?;
                            if child_injection.is_empty() {
                                let for_parse = TreeParser::parse_for_block(
                                    &input,
                                    &init_type,
                                    is_first_component,
                                )?;
                                if for_parse.is_empty() {
                                    return Err(syn::Error::new(
                                        input.span().unwrap().into(),
                                        "didn't expect this here",
                                    ));
                                }
                                for_parse
                            } else {
                                child_injection
                            }
                        } else {
                            conditional_block
                        }
                    } else {
                        component_block
                    };
                    // now no longer any component can be root
                    is_first_component = false;
                    // there can only be one root component
                    has_one_root_component = true;
                    // there can only be one child, therefore after parsing a component
                    // it is guaranteed that the next component will be sibling
                    init_type = InitType::Sibling;
                    parsed
                } else {
                    execution_block
                }
            };

            tree = quote! { #tree #parsed };
            // there has to be a comma if input is not empty and the previous parse was successful
            if !input.is_empty() {
                input.parse::<syn::token::Comma>()?;
            }
        }
    }
}
