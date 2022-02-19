use quote::{quote, ToTokens, TokenStreamExt};

pub enum LifeTime {
    Rc(Option<Context>),
    Context(Context),
    Simple,
}

impl ToTokens for LifeTime {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            LifeTime::Rc(Some(ctx)) | LifeTime::Context(ctx) => ctx.to_tokens(tokens),
            _ => (),
        }
    }
}

impl LifeTime {
    pub fn get_context(self) -> Option<Context> {
        match self {
            LifeTime::Rc(Some(ctx)) | LifeTime::Context(ctx) => Some(ctx),
            _ => None,
        }
    }

    pub fn requires_context(&self) -> bool {
        matches!(self, LifeTime::Rc(Some(_)) | LifeTime::Context(_))
    }
}

pub enum Context {
    Push,
    Absorb,
}

impl ToTokens for Context {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.append_all(match self {
            Context::Push => quote! {
                __ctx.push(Box::new(__child));
            },
            Context::Absorb => quote! {
                __ctx.absorb(__child);
            },
        });
    }
}
