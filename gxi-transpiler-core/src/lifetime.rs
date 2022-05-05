use quote::{quote, ToTokens, TokenStreamExt};

pub enum LifeTime {
    Context(ContextAction),
    Constant,
}

impl Default for LifeTime {
    fn default() -> Self {
        Self::Constant
    }
}

pub enum ContextAction {
    Push,
    Absorb,
}

impl Default for ContextAction {
    fn default() -> Self {
        Self::Push
    }
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
