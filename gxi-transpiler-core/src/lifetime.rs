use quote::{quote, ToTokens, TokenStreamExt};

pub enum LifeTime {
    Context(ContextType),
    Constant,
}

impl Default for LifeTime {
    fn default() -> Self {
        Self::Constant
    }
}

/// refer to [`gxi::context`]
pub enum ContextType {
    /// refer to [`gxi::ConstantContext`]
    Constant(ConstantContextAction),
    /// refer to [`gxi::IndexedContext`]
    Indexed,
}

impl Default for ContextType {
    fn default() -> Self {
        Self::Constant(Default::default())
    }
}

pub enum ConstantContextAction {
    Push,
    Absorb,
}

impl Default for ConstantContextAction {
    fn default() -> Self {
        Self::Push
    }
}

impl ToTokens for ConstantContextAction {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.append_all(match self {
            ConstantContextAction::Push => quote! {
                __ctx.push(Box::new(__child));
            },
            ConstantContextAction::Absorb => quote! {
                __ctx.absorb(__child);
            },
        });
    }
}
