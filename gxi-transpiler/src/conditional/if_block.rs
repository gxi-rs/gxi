use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

use crate::{
    optional_parse::{impl_parse_for_optional_parse, OptionalParse},
    scope::Scope,
};
use else_arm::ElseArm;

use self::if_arm::IfArm;

pub struct IfBlock {
    pub if_arm: IfArm,
    pub scope: Scope,
    // number of conditional arms
    pub depth: usize,
}

impl OptionalParse for IfBlock {
    fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
        let mut if_arm = if let Some(if_arm) = IfArm::optional_parse(input)? {
            if_arm
        } else {
            return Ok(None);
        };

        // there is 1 if block
        let mut depth = 1usize;

        {
            let mut if_arm_ = &if_arm;
            loop {
                match &*if_arm_.else_arm {
                    ElseArm::WithIfArm { if_arm, .. } => {
                        if !if_arm.scope.is_const() {
                            return Err(syn::Error::new(
                                if_arm.if_token.span(),
                                "chaining observables is unsupported. consider adding const here",
                            ));
                        }
                        depth += 1;
                        if_arm_ = &if_arm
                    }
                    ElseArm::PureElseArm { .. } => {
                        depth += 1;
                        break;
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        // NOTE: turn scope of first block to const
        let scope = if_arm.scope;
        if_arm.scope = Scope::Constant;

        Ok(Some(Self {
            if_arm,
            depth,
            scope,
        }))
    }
}
// __index_offset = index of first if statement
// formula = Math::min(__index_offset, INDEX);
// __index_offset+=1;

impl IfBlock {
    pub fn to_tokens(&self, tokens: &mut TokenStream2, node_index: usize) {
        let mut if_arm_tokens =
            self.if_arm
                .to_token_stream(node_index, 1, self.depth, self.scope.is_const());

        if !self.scope.is_const() {
            if_arm_tokens = quote! {
                use std::ops::{DerefMut, Deref};

                let mut __if_counter = __if_counter.deref().borrow_mut();
                #if_arm_tokens
            };
        }

        let mut main_body = self
            .scope
            .to_token_stream(&if_arm_tokens, &quote! {gxi::Element});

        if let Scope::Observable(_) = self.scope {
            main_body = quote! {
                let __if_counter = State::new(0usize);
                #main_body
            }
        }

        tokens.append_all(main_body);
    }
}

impl_parse_for_optional_parse!(IfBlock);

mod if_arm {
    use crate::{
        component::NodeBlock,
        conditional::if_block::{body_to_tokens, else_arm::ElseArm},
    };
    use quote::quote;
    use syn::__private::TokenStream2;

    use crate::{
        optional_parse::{impl_parse_for_optional_parse, OptionalParse},
        scope::Scope,
    };

    pub struct IfArm {
        pub scope: Scope,
        pub if_token: syn::Token!(if),
        pub body: NodeBlock,
        pub else_arm: Box<ElseArm>,
        pub condition: syn::Expr,
    }

    impl OptionalParse for IfArm {
        fn optional_parse(input: &syn::parse::ParseStream) -> syn::Result<Option<Self>> {
            let if_token = if let Ok(if_token) = input.parse::<syn::Token!(if)>() {
                if_token
            } else {
                return Ok(None);
            };

            // get scope
            let (condition, scope) = {
                let is_const = input.parse::<syn::Token!(const)>().is_ok();
                let condition = input.parse::<syn::Expr>()?;
                // no need to check scope when const keyword is provided
                let scope = if is_const {
                    Scope::Constant
                } else {
                    Scope::find_expr_scope(&condition)?
                };
                (condition, scope)
            };

            // parse children
            let body = {
                let syn::group::Braces { content, .. } = syn::group::parse_braces(&input)?;

                content.parse()?
            };

            // else arm
            let else_arm = input.parse::<ElseArm>()?.into();

            Ok(Some(Self {
                scope,
                if_token,
                body,
                else_arm,
                condition,
            }))
        }
    }

    impl_parse_for_optional_parse!(IfArm);

    impl IfArm {
        pub fn to_token_stream(
            &self,
            node_index: usize,
            branch_index: usize,
            depth: usize,
            constant_scope: bool,
        ) -> TokenStream2 {
            let Self {
                if_token,
                body,
                else_arm,
                condition,
                ..
            } = self;

            let body = body_to_tokens(
                &quote! {#body},
                branch_index,
                node_index,
                depth,
                constant_scope,
            );

            let else_arm =
                else_arm.to_token_stream(node_index, branch_index + 1, depth, constant_scope);

            quote! {
                #if_token #condition {
                    #body
                } #else_arm
            }
        }
    }
}

mod else_arm {
    use crate::component::NodeBlock;
    use crate::optional_parse::OptionalParse;
    use quote::quote;
    use syn::__private::TokenStream2;
    use syn::parse::Parse;

    use super::body_to_tokens;
    use super::if_arm::IfArm;

    // NOTE: can't exist independently
    pub enum ElseArm {
        WithIfArm {
            else_token: syn::Token!(else),
            if_arm: IfArm,
        },
        PureElseArm {
            else_token: syn::Token!(else),
            body: NodeBlock,
        },
        None,
    }

    impl Parse for ElseArm {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            if let Ok(else_token) = input.parse::<syn::Token!(else)>() {
                if let Some(if_arm) = IfArm::optional_parse(&input)? {
                    Ok(Self::WithIfArm { else_token, if_arm })
                } else {
                    let syn::group::Braces { content, .. } = syn::group::parse_braces(&input)?;
                    Ok(Self::PureElseArm {
                        else_token,
                        body: content.parse()?,
                    })
                }
            } else {
                Ok(Self::None)
            }
        }
    }

    impl ElseArm {
        pub fn to_token_stream(
            &self,
            node_index: usize,
            branch_index: usize,
            depth: usize,
            constant_scope: bool,
        ) -> TokenStream2 {
            match self {
                ElseArm::WithIfArm { else_token, if_arm } => {
                    let if_tokens =
                        if_arm.to_token_stream(node_index, branch_index, depth, constant_scope);
                    quote! { #else_token #if_tokens }
                }
                ElseArm::PureElseArm {
                    else_token,
                    body: block,
                } => {
                    let body = body_to_tokens(
                        &quote! {#block},
                        branch_index,
                        node_index,
                        depth,
                        constant_scope,
                    );
                    quote! {
                        #else_token #body
                    }
                }
                ElseArm::None => {
                    if constant_scope {
                        TokenStream2::new()
                    } else {
                        let body = body_to_tokens(
                            &TokenStream2::new(),
                            branch_index,
                            node_index,
                            depth,
                            constant_scope,
                        );
                        quote! {
                            else {
                                #body
                            }
                        }
                    }
                }
            }
        }

        pub fn is_some(&self) -> bool {
            if let Self::None = self {
                false
            } else {
                true
            }
        }
    }
}

fn body_to_tokens(
    body: &TokenStream2,
    branch_index: usize,
    node_index: usize,
    depth: usize,
    constant_scope: bool,
) -> TokenStream2 {
    if constant_scope {
        quote! {
            __node.push(#body);
        }
    } else {
        // no trailing else arm
        let body = if body.is_empty() {
            quote! {
                None
            }
        } else {
            quote! {
                Some(#body)
            }
        };
        quote! {
            if *__if_counter != #branch_index {
                __node.set_at_index(
                    #body,
                    #node_index,
                    *__if_counter == 0 || *__if_counter == (#depth + 1)
                );
                *__if_counter = #branch_index;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::{quote, ToTokens};

    use crate::{
        conditional::{if_block::else_arm::ElseArm, IfBlock},
        scope::Scope,
    };

    #[test]
    fn conditional_if_block() -> syn::Result<()> {
        let const_condition_expr = quote! { 3 == 4};
        let condition_expr = quote! { t == 3 && #const_condition_expr };
        {
            let expr = quote! { if #condition_expr { div } else { a } };

            let IfBlock {
                depth,
                if_arm,
                scope,
            } = syn::parse2(expr)?;

            assert_eq!(scope, Scope::Observable(quote! {t}));
            assert_eq!(
                if_arm.condition.to_token_stream().to_string(),
                condition_expr.to_string()
            );
            assert!(matches!(*if_arm.else_arm, ElseArm::PureElseArm { .. }));
            assert_eq!(depth, 2);
        }
        {
            let expr = quote! { if #condition_expr { div } else if #const_condition_expr { a } };

            let IfBlock {
                depth,
                if_arm,
                scope,
            } = syn::parse2(expr)?;

            assert_eq!(scope, Scope::Observable(quote! {t}));
            assert_eq!(
                if_arm.condition.to_token_stream().to_string(),
                condition_expr.to_string()
            );
            assert_eq!(depth, 2);

            let else_arm = &*if_arm.else_arm;
            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
                assert_eq!(if_arm.scope, Scope::Constant);
                assert_eq!(
                    if_arm.condition.to_token_stream().to_string(),
                    const_condition_expr.to_string()
                );
                assert_eq!(if_arm.else_arm.is_some(), false);
            } else {
                panic!("expected ElseArm::WithIfArm")
            }
        }
        {
            let expr = quote! { if #condition_expr { div } else if const #const_condition_expr { a } else { a } };

            let IfBlock {
                depth,
                if_arm,
                scope,
            } = syn::parse2(expr)?;

            assert_eq!(scope, Scope::Observable(quote! {t}));
            assert_eq!(
                if_arm.condition.to_token_stream().to_string(),
                condition_expr.to_string()
            );
            let else_arm = &*if_arm.else_arm;
            if let ElseArm::WithIfArm { if_arm, .. } = else_arm {
                assert_eq!(if_arm.scope, Scope::Constant);
                assert_eq!(
                    if_arm.condition.to_token_stream().to_string(),
                    const_condition_expr.to_string()
                );
                assert!(matches!(*if_arm.else_arm, ElseArm::PureElseArm { .. }));
            } else {
                panic!("expected ElseArm::WithIfArm")
            }
            assert_eq!(depth, 3);
        }
        Ok(())
    }
}
