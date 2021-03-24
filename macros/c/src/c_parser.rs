use quote::{*};
use syn::{*};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

enum InitType {
    Child,
    Sibling,
}

pub struct CParser {
    pub tree: TokenStream2
}

impl CParser {
    fn custom_parse(input: ParseStream, init_type: InitType) -> Result<Self> {
        if let Ok(block) = input.parse::<Block>() {
            let content = CParser::custom_parse(&input, InitType::Sibling).unwrap();
            let content_tree = content.tree;
            return Ok(CParser {
                tree: quote! {
                let node = {
                    let (node , cont) = {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let cont = Rc::clone(&cont);
                        (node_borrow.init_sibling(Box::new(move || Pure::new(cont.clone())), false).0, node.clone())
                    };
                    let state = state.as_ref().borrow();
                    { #block }
                    node
                };
                #content_tree
            }
            });
        }
        //not mandatory to have a bracket or component inside the macro. macro can be empty
        if let Ok(name) = input.parse::<Ident>() {
            //check for block
            let mut tree = {
                let block = if let Ok(block) = input.parse::<Block>() { block.to_token_stream() } else { (quote! {{}}).into() };
                if let InitType::Child = init_type { quote! { n!(#name init_child #block); } } else { quote! { n!(#name init_sibling #block); } }
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
            return Ok(CParser { tree });
        }
        Ok(CParser { tree: TokenStream2::new() })
    }
}

impl Parse for CParser {
    fn parse(input: ParseStream) -> Result<Self> {
        CParser::custom_parse(input, InitType::Child)
    }
}