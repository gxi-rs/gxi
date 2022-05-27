use super::{wrapper::NodeWrapper, NodeSubBlock};
use crate::{
    blocks::{
        conditional::ConditionalBlock,
        node::{NodeProps, NodeSubTree},
    },
    lifetime::{ConstantContextAction, ContextType, LifeTime},
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    state::{State, StateExt},
    sub_tree::{SubTree, NodeSubTreeExt},
};
use quote::ToTokens;
use quote::{quote, TokenStreamExt};
use syn::Token;
use syn::__private::TokenStream2;
use syn::parse::ParseStream;

/// **NodeBlock** contructs a node by parsing different
/// [`NodeType Syntax Definations`](NodeType) and [`its' subtree`](NodeSubTree).
/// Based on the 2 factors the final [`LifeTime`] is calculated.
pub struct NodeBlock {
    /// A node block can be of 3 different [`NodeTypes`](NodeType).
    pub node_type: NodeType,
    /// Child elements of the tree, a contained in `Square Brackets`
    ///
    /// ```rust
    /// #NodeType [
    ///     ...NodeSubTree
    /// ]
    /// ```
    pub sub_tree: NodeSubTree,
    /// LifeTime of the node depending on the [`NodeType`](NodeType)
    /// and [`NodeSubTree`]
    ///
    /// ## Effects of NodeSubTree
    /// If any [`NodeSubBlock`](NodeSubBlock) in the first depth of the tree is
    ///
    /// 1. [`ConditionalBlock`]
    ///     with [`State::Observable`] then the lifetime is escalated to
    ///      [`LifeTime::Context`] and wrapper to [`NodeWrapper::Rc`]
    pub lifetime: LifeTime,
    pub wrapper: NodeWrapper,
}

impl OptionalParse for NodeBlock {
    fn optional_parse(input: &ParseStream) -> syn::Result<Option<Self>> {
        let node_type = if let Some(node_type) = NodeType::parse(input)? {
            node_type
        } else {
            return Ok(None);
        };
        // parse children
        let sub_tree =
            if let Ok(syn::group::Brackets { content, .. }) = syn::group::parse_brackets(input) {
                if !content.is_empty() {
                    NodeSubTree::parse(&content)?
                } else {
                    Default::default()
                }
            } else {
                Default::default()
            };

        let mut wrapper = NodeWrapper::None;
        let mut lifetime = LifeTime::from(&node_type);

        // set wrapper = NodeWrapper::Rc and LifeTime = Context
        // is ConditionalBlock is found with State::Observable
        for sub_node in sub_tree.iter() {
            if let NodeSubBlock::Conditional(ConditionalBlock::If(if_block)) = sub_node {
                if let State::Observable(_) = if_block.state {
                    wrapper = NodeWrapper::Rc;
                    lifetime = LifeTime::Context(Default::default());
                    break;
                }
            }
        }

        Ok(Some(Self {
            node_type,
            sub_tree,
            lifetime,
            wrapper,
        }))
    }
}

impl_parse_for_optional_parse!(NodeBlock);

impl ToTokens for NodeBlock {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            sub_tree: subtree,
            node_type,
            lifetime: _,
            wrapper,
        } = self;

        let init_call = node_type.get_init_call();

        let mid_calls = match node_type {
            // functional components can't have props
            NodeType::FunctionalComponent { .. } => quote! {},
            _ => {
                let (const_props, observable_props) = node_type.get_const_and_observable_props();

                let mut subtree_tokens = TokenStream2::new();
                subtree.to_tokens(&mut subtree_tokens);

                quote! {
                    #const_props

                    #subtree_tokens

                    #observable_props
                }
            }
        };

        let rc_token = wrapper.to_token_stream();

        // assemble
        if rc_token.is_empty() && mid_calls.is_empty() {
            tokens.append_all(quote! {
                let __child = #init_call;
            })
        } else {
            tokens.append_all(quote! {
                let __child = {
                    let mut __node = #init_call;
                    #rc_token

                    #mid_calls

                    __node
                };
            });
        }
    }
}

impl StateExt for NodeBlock {
    fn get_state(&self) -> State {
        if let Some(props) = self.node_type.get_props() {
            State::from(props)
        } else {
            State::Constant
        }
    }

    fn get_nested_state(&self) -> State {
        self.sub_tree.get_nested_state()
    }
}

type Arg = syn::Expr;

/// A node block can be of 3 different [`NodeTypes`](NodeType).
/// Each with it's distinct syntax.
pub enum NodeType {
    /// # Syntax
    ///
    /// ```
    /// $path::$constructor($args?..)($props?..)
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// mycrate::Head::new(20, 30)(size = "big")
    /// ```
    ///
    /// - `$path`        = `mycrate::Head`
    /// - `$constructor` = `new`
    /// - `$args`        = `20, 30`
    /// - `$props`       = `size = big`
    Component {
        /// path preceding the constructor.
        /// At least 1 path component is required
        path: TokenStream2,
        /// associated function responsible to create the component
        constructor: TokenStream2,
        /// *optional*
        ///
        /// punctated(,) list of values inside parenthesis passed
        /// to constructor function, as it is.
        args: Vec<Arg>,
        /// *optional*
        props: NodeProps,
    },
    /// # Syntax
    ///
    /// ```
    /// $path?::$constructor($args?..)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// gxi! {
    ///     hello::world(20, 32)
    /// }
    /// ```
    ///
    /// - `$path`           = `hello`
    /// - `$constructor`    = `world`
    /// - `$args`           = `20, 32`
    FunctionalComponent {
        /// *optional*
        ///
        /// path preceding the constructor
        path: TokenStream2,
        /// name of the functional component
        constructor: TokenStream2,
        /// *optional*
        ///
        /// punctated(,) list of values inside parenthesis passed
        /// to constructor function, as it is.
        args: Vec<Arg>,
    },
    /// # Syntax
    ///
    /// ```
    /// $name($props?..)
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// gxi! {
    ///     h1(value = 20, game="web dev")
    /// }
    /// ```
    ///
    /// - `$name`  = `h1`
    /// - `$props` = `value = 20, game = "web dev"`
    ///
    /// > Note: If the props parenthesis is empty and no path precedes `$name`
    /// > then it is considered as [`NodeType::FunctionalComponent`]
    Element {
        /// `gxi::Element("$name")`
        name: String,
        /// *optional*
        props: NodeProps,
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
                quote! { #path #constructor(#(#args),*) }
            }
            NodeType::Element { name, .. } => quote! { gxi::Element::from(#name) },
        }
    }

    pub fn get_const_and_observable_props(&self) -> (TokenStream2, TokenStream2) {
        let mut const_props = TokenStream2::new();
        let mut observable_props = TokenStream2::new();

        if let Some(props) = self.get_props() {
            for prop in props.iter() {
                if let State::Constant = prop.state {
                    prop.to_tokens(&mut const_props);
                } else {
                    prop.to_tokens(&mut observable_props);
                }
            }
        }

        (const_props, observable_props)
    }

    pub fn get_props(&self) -> Option<&NodeProps> {
        match self {
            NodeType::FunctionalComponent { .. } => None,
            NodeType::Element { props, .. } | NodeType::Component { props, .. } => Some(props),
        }
    }
}

impl From<&NodeType> for LifeTime {
    fn from(node_type: &NodeType) -> Self {
        if let NodeType::FunctionalComponent { .. } = node_type {
            return LifeTime::Context(ContextType::Constant(ConstantContextAction::Absorb));
        }

        if let Some(props) = node_type.get_props() {
            for prop in props.iter() {
                if let LifeTime::Context(_) = prop.lifetime {
                    return LifeTime::Context(ContextType::Constant(ConstantContextAction::Push));
                }
            }
        }

        LifeTime::Constant
    }
}


impl From<&NodeType> for NodeWrapper {
    fn from(_: &NodeType) -> Self {
        todo!()
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
                                props.push(content.parse()?);
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
                ensure!(props.len() == 0);
            } else {
                bail!("wrong node type")
            }
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
            let node_type = syn::parse2::<NodeTypeParser>(quote! { Comp::with_name("hey") })?.0;
            if let NodeType::Component {
                args,
                props,
                path,
                constructor,
            } = &node_type
            {
                ensure!(constructor.to_string() == "with_name");
                ensure!(path.to_string() == quote! {Comp}.to_string());
                ensure!(props.is_empty() == true);
                ensure!(args.len() == 1);
            } else {
                bail!("wrong node type")
            }
            ensure!(
                node_type.get_init_call().to_string()
                    == quote! {Comp::with_name("hey")}.to_string()
            );
        }
        Ok(())
    }
}
