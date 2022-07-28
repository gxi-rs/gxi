pub use context::*;
pub use imports::*;

mod context {
    use quote::{quote, ToTokens, TokenStreamExt};

    pub enum ContextSnippets {
        /// IndexedContext
        Indexed,
    }

    impl ToTokens for ContextSnippets {
        fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
            match self {
                Self::Indexed => tokens.append_all(quote! {
                    let mut __ctx = gxi::IndexedContext::default();
                }),
            }
        }
    }
}

mod imports {
    use quote::{quote, ToTokens, TokenStreamExt};

    pub enum Imports {
        Deref,
    }

    impl ToTokens for Imports {
        fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
            match self {
                Self::Deref => tokens.append_all(quote! {
                    use std::ops::{DerefMut, Deref};
                }),
            }
        }
    }
}
