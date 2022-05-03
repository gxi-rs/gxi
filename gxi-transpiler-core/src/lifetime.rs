use quote::{quote, ToTokens, TokenStreamExt};

/// refer to [`gxi::VNodeContext`]
pub enum LifeTime {
    Context(ContextAction),
    Constant,
}

pub enum ContextAction {
    Push,
    Absorb,
}

impl ToTokens for ContextAction {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.append_all(match self {
            ContextAction::Push => quote! {
                __ctx.push(Box::new(__child));
            },
            ContextAction::Absorb => quote! {
                __ctx.absorb(__child);
            },
        });
    }
}
