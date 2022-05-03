use quote::{quote, ToTokens, TokenStreamExt};

pub enum NodeWrapper {
    Rc,
    None,
}

impl Default for NodeWrapper {
    fn default() -> Self {
        Self::None
    }
}

impl ToTokens for NodeWrapper {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            Self::Rc => tokens.append_all(quote! {
                let __node = std::rc::Rc::new(__node);
            }),
            Self::None => (),
        }
    }
}
