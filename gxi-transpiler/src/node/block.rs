use super::{NodeProps, NodeSubTree};
use crate::optional_parse::{impl_parse_for_optional_parse, OptionalParse};
use crate::scope::Scope;
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::Token;
use syn::__private::TokenStream2;
use syn::parse::ParseStream;

type Arg = syn::Expr;

pub enum NodeType {
    /// name(args..)(props..)
    FunctionalComponent {
        args: Vec<Arg>,
        path: TokenStream2,
        constructor: TokenStream2,
    },
    /// gxi::Element
    /// name(props..)
    Element { name: String, props: NodeProps },
    /// Name::constructor(args..)(props..)
    Component {
        args: Vec<Arg>,
        props: NodeProps,
        path: TokenStream2,
        constructor: TokenStream2,
    },
}

impl NodeType {
    pub fn get_init_call(&self) -> TokenStream2 {
        match self {
            NodeType::FunctionalComponent {
                args,
                path,
                constructor,
            }
            | NodeType::Component {
                args,
                path,
                constructor,
                ..
            } => {
                let mut path = path.to_token_stream();
                if !path.is_empty() {
                    path.append_all(quote! {::})
                };
                quote! { #path#constructor(#(#args),*) }
            }
            NodeType::Element { name, .. } => quote! { gxi::Element::from(#name) },
        }
    }

    pub fn get_return_type(&self) -> TokenStream2 {
        match self {
            NodeType::FunctionalComponent { .. } => {
                unreachable!("Internal Error: functional components have unknown return type")
            }
            NodeType::Element { .. } => quote! {gxi::Element},
            NodeType::Component { path, .. } => path.to_token_stream(),
        }
    }

    pub fn get_const_and_observable_props(
        &self,
        return_type: &TokenStream2,
    ) -> (TokenStream2, TokenStream2) {
        let mut const_props = TokenStream2::new();
        let mut observable_props = TokenStream2::new();
        let props = self.get_props();

        for prop in &props.props {
            if let Scope::Constant = prop.scope {
                prop.to_tokens(&mut const_props, return_type);
            } else {
                prop.to_tokens(&mut observable_props, return_type);
            }
        }

        (const_props, observable_props)
    }

    pub fn get_props(&self) -> &NodeProps {
        match self {
            NodeType::FunctionalComponent { .. } => {
                unreachable!("Internal Error: functional component's can't have props.")
            }
            NodeType::Element { props, .. } | NodeType::Component { props, .. } => props,
        }
    }
}

fn starts_with_lower_case(string: &str) -> bool {
    matches!(string.chars().next().unwrap(), 'a'..='z')
}

impl NodeType {
    fn parse(input: ParseStream) -> syn::Result<Option<Self>> {
        if input.is_empty() {
            return Err(syn::Error::new(input.span(), "expected tokens"));
        }
        #[allow(clippy::question_mark)]
        let mut path = if let Ok(path) = input.parse::<syn::Path>() {
            path
        } else {
            return Ok(None);
        };

        let last_segment = path.segments.last().unwrap();
        let last_segment = last_segment.ident.to_string();

        // if there is only one path segment and name starts with a lower case character then
        // it's a NativeElement

        // check if last_segment starts with a lower case character

        return Ok(Some(
            // component with no explicit function call
            if !starts_with_lower_case(&last_segment) {
                Self::Component {
                    args: Default::default(),
                    props: input.parse()?,
                    path: path.to_token_stream(),
                    constructor: quote! {new},
                }
            } else {
                // starts with lower case
                if path.segments.len() == 1 {
                    // only element or function
                    // if props then element else function
                    let mut props = NodeProps::default();

                    if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(input)
                    {
                        if content.peek(Token!(const)) || content.peek2(Token!(=)) {
                            // parse props
                            // WARN: code duplication
                            while !content.is_empty() {
                                props.props.push(content.parse()?);
                                if !content.is_empty() {
                                    content.parse::<syn::token::Comma>()?;
                                }
                            }
                        } else {
                            let mut args = Vec::<Arg>::default();
                            while !content.is_empty() {
                                args.push(content.parse()?);
                                if !content.is_empty() {
                                    content.parse::<syn::token::Comma>()?;
                                }
                            }
                            return Ok(Some(Self::FunctionalComponent {
                                args,
                                path: TokenStream2::new(),
                                constructor: path.to_token_stream(),
                            }));
                        }
                    }

                    Self::Element {
                        name: last_segment,
                        props,
                    }
                } else {
                    // only function or component with constructor
                    // if second last is capital then component
                    // otherwise function

                    let mut args = Vec::<Arg>::default();

                    if let Ok(syn::group::Parens { content, .. }) = syn::group::parse_parens(input)
                    {
                        while !content.is_empty() {
                            args.push(content.parse()?);

                            if !content.is_empty() {
                                content.parse::<syn::Token!(,)>()?;
                            }
                        }
                    }

                    let constructor = path.segments.pop().unwrap().into_value().to_token_stream();
                    let second_last_starts_with_upper_case =
                        !starts_with_lower_case(&path.segments.last().unwrap().ident.to_string());
                    // remove trailing ::
                    let mut path = path.to_token_stream().to_string();
                    if path.ends_with("::") {
                        path.pop();
                        path.pop();
                    }

                    let path = syn::parse_str(&path)?;

                    // if second last segment starts with upper case then it a standard component
                    if second_last_starts_with_upper_case {
                        Self::Component {
                            args,
                            props: input.parse()?,
                            path,
                            constructor,
                        }
                    } else {
                        Self::FunctionalComponent {
                            args,
                            path,
                            constructor,
                        }
                    }
                }
            },
        ));
    }
}

#[doc = include_str ! ("./README.md")]
pub struct NodeBlock {
    pub node_type: NodeType,
    pub subtree: NodeSubTree,
}

impl OptionalParse for NodeBlock {
    fn optional_parse(input: &ParseStream) -> syn::Result<Option<Self>> {
        let node_type = if let Some(node_type) = NodeType::parse(input)? {
            node_type
        } else {
            return Ok(None);
        };
        // parse children
        let subtree =
            if let Ok(syn::group::Brackets { content, .. }) = syn::group::parse_brackets(input) {
                if !content.is_empty() {
                    content.parse::<NodeSubTree>()?
                } else {
                    Default::default()
                }
            } else {
                Default::default()
            };

        Ok(Some(Self { node_type, subtree }))
    }
}

impl_parse_for_optional_parse!(NodeBlock);

/// Optimization Rules:
/// 1. If a component consists of a serializable sub tree then serialize them to string
///
impl ToTokens for NodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self { subtree, node_type } = self;

        let init_call = node_type.get_init_call();

        // functional components can't have props

        let mid_calls = match node_type {
            NodeType::FunctionalComponent { .. } => TokenStream2::new(),
            _ => {
                let return_type = node_type.get_return_type();
                let (const_props, observable_props) =
                    node_type.get_const_and_observable_props(&return_type);

                let mut subtree_tokens = TokenStream2::new();
                subtree.to_tokens(&mut subtree_tokens, &return_type);

                quote! {
                    #const_props

                    #subtree_tokens

                    #observable_props
                }
            }
        };
        // assemble
        tokens.append_all(quote! {{
            use gxi::{VNode, VContainerWidget};

            let mut __node = #init_call;

            #mid_calls

            __node
        }});
    }
}

impl ToString for NodeBlock {
    /// if to string is called it means that the whole sub tree is serializable
    fn to_string(&self) -> String {
        if cfg!(feature = "web") {
        } else {
            unreachable!("Can't serialize with the current feature flag. Most likely an internal error. Please use the github issue tracker https://github.com/gxi-rs/gxi/issues")
        }
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::NodeType;
    use anyhow::{bail, ensure};
    use quote::quote;
    use syn::parse::Parse;

    struct NodeTypeParser(NodeType);

    impl Parse for NodeTypeParser {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            Ok(Self(NodeType::parse(input)?.unwrap()))
        }
    }

    #[test]
    fn node_type() -> anyhow::Result<()> {
        {
            let node_type = syn::parse2::<NodeTypeParser>(quote! { Body })?.0;
            if let NodeType::Component {
                constructor,
                path,
                args,
                props,
            } = &node_type
            {
                ensure!(constructor.to_string() == "new");
                ensure!(path.to_string() == "Body");
                ensure!(args.len() == 0);
                ensure!(props.props.len() == 0);
            } else {
                bail!("wrong node type")
            }
            let return_type = node_type.get_return_type();
            ensure!(return_type.to_string() == "Body");
            ensure!(node_type.get_init_call().to_string() == quote! {Body::new()}.to_string());
        }
        {
            let node_type = syn::parse2::<NodeTypeParser>(quote! { func(12, 12) })?.0;
            if let NodeType::FunctionalComponent {
                constructor,
                path,
                args,
            } = &node_type
            {
                ensure!(constructor.to_string() == "func");
                ensure!(path.is_empty() == true);
                ensure!(args.len() == 2);
            } else {
                bail!("wrong node type")
            }
            ensure!(node_type.get_init_call().to_string() == quote! {func(12, 12)}.to_string());
        }
        {
            let node_type =
                syn::parse2::<NodeTypeParser>(quote! { Comp::with_name("hey") })?.0;
            if let NodeType::Component {
                args,
                props,
                path,
                constructor,
            } = &node_type
            {
                ensure!(constructor.to_string() == "with_name");
                ensure!(path.to_string() == quote! {Comp}.to_string());
                ensure!(props.props.is_empty() == true);
                ensure!(args.len() == 1);
            } else {
                bail!("wrong node type")
            }
            let return_type = node_type.get_return_type();
            ensure!(return_type.to_string() == quote! {Comp}.to_string());
            ensure!(
                node_type.get_init_call().to_string()
                    == quote! {Comp::with_name("hey")}.to_string()
            );
        }
        Ok(())
    }
}
