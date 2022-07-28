use std::collections::HashSet;
use std::ops::{Deref, DerefMut, Range};

use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

#[derive(Debug, Default, Clone)]
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
    pub fn borrowed_token_stream(&self, range: &Range<usize>) -> TokenStream2 {
        let slice = &self[range.to_owned()];

        if self.len() == 1 {
            let observable = &slice[0];

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

    /// removes duplicate observables
    pub fn remove_duplicates(&mut self) {
        let mut set = HashSet::with_capacity(self.len());

        self.0.retain(|v| set.insert(v.to_string()))
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

impl PartialEq for Observables {
    //NOTE: both should have no duplicates
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let mut set = HashSet::with_capacity(self.len());

        for ele in &self.0 {
            set.insert(ele.to_string());
        }

        for ele in &other.0 {
            if set.get(&ele.to_string()).is_none() {
                return false;
            }
        }

        true
    }
}
