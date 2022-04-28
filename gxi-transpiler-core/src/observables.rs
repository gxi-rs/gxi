use std::ops::{Deref, DerefMut};

use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

#[derive(Debug, Default)]
pub struct Observables(pub Vec<TokenStream2>);

impl Observables {
    /// Creates token stream with borrow syntax for all Observables
    /// ```rust
    /// let observable = (**observable).borrow();
    /// ```
    pub fn borrowed_token_stream(&self) -> TokenStream2 {
        let mut scoped_variables_borrow = TokenStream2::new();

        for observable in self.iter() {
            scoped_variables_borrow.append_all(quote! {
                let #observable = (**#observable).borrow();
            });
        }

        scoped_variables_borrow
    }
}

impl Deref for Observables {
    type Target = Vec<TokenStream2>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Observables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
