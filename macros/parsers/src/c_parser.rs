use quote::*;
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::*;

use crate::{InitType, NParser};

pub struct CParser {
    pub tree: TokenStream2,
}

impl CParser {
    fn custom_parse(input: ParseStream, init_type: InitType) -> TokenStream2 {
        parse_execution_block(&input);
        //not mandatory to have a bracket or component inside the macro. macro can be empty
        if let Ok(name) = input.parse::<Ident>() {
            //check for block
            let mut tree = {
                let block = if let Ok(block) = input.parse::<Block>() {
                    block.to_token_stream()
                } else {
                    (quote! {{}}).into()
                };
                match init_type {
                    InitType::Child => {
                        NParser::parse(quote! { #name init_child #block }.into())
                            .unwrap()
                            .tree
                    }
                    InitType::Sibling => (quote! { n!(#name init_sibling #block); }),
                    InitType::Pure(i) => quote! { n!(#i #name init_child #block); },
                }
            };
            {
                //check for first block
                match group::parse_brackets(&input) {
                    syn::__private::Ok(brackets) => {
                        let content = CParser::parse(&brackets.content).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree {  let cont = node.clone(); #content_tree } };
                    }
                    syn::__private::Err(_error) => {}
                }
                //parse ,
                match input.parse::<syn::Token![,]>() {
                    Ok(_) => {
                        let content = CParser::custom_parse(&input, InitType::Sibling).unwrap();
                        let content_tree = content.tree;
                        tree = quote! { #tree #content_tree };
                    }
                    _ => {}
                }
            }
            return tree;
        }
        TokenStream2::new()
    }
}

impl Parse for CParser {
    fn parse(input: ParseStream) -> Result<Self> {
        let init_type = if let Ok(i) = input.parse::<syn::Lit>() {
            if let Lit::Int(i) = i {
                let i = i.base10_parse().unwrap();
                if i > 0 {
                    InitType::Pure(i)
                }
                panic!("Expected an u32 greater than 1")
            }
            panic!("Expected an u32")
        } else {
            InitType::Child
        };
        Ok(CParser {
            tree: CParser::custom_parse(input, init_type),
        })
    }
}

fn parse_execution_block(input: &ParseStream) -> TokenStream2 {
    if let Ok(block) = input.parse::<Block>() {
        let content = CParser::custom_parse(input, InitType::Sibling).unwrap();
        let content_tree = content.tree;
        let init_type = match init_type {
            InitType::Child => (quote! {init_child}),
            _ => (quote! {init_sibling}),
        };
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
            #content_tree
        };
    };
    return TokenStream2::new();
}
