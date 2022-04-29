use std::ops::{Deref, DerefMut};

use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

#[derive(Debug, Default)]
pub struct Observables(pub Vec<TokenStream2>);

impl Observables {
    /// Creates token stream with borrow syntax for all Observables
    /// iff single observable then refcell is borrowed
    /// ```rust
    /// let observable = observable.borrow();
    /// ```
    /// else multi observable pattern is used which require Rc deref and refcell borrow
    /// ```rust
    /// let observable = (**observable).borrow();
    /// ```
    pub fn borrowed_token_stream(&self) -> TokenStream2 {
        if self.len() == 1 {
            let observable = &self[0];

            return quote! {
              let #observable = #observable.borrow();
            };
        }

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
