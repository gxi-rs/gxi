use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

use crate::observables::Observables;

pub struct ObserverBuilder<'a> {
    pub pre_add_observer_tokens: &'a TokenStream2,
    pub add_observer_body_tokens: &'a TokenStream2,
    /// if true and one observable
    /// then `.borrow()` is called on the `RefCell` closure value
    /// else if true and more than one observables
    /// then
    pub borrow: bool,
}

impl<'a> ObserverBuilder<'a> {
    /// if there is only one observable
    /// add_observer is called on the observable with a closure `|RefCell|`
    ///
    /// ## Args
    /// * `observables` - a non empty array of identifiers whose value has to observed
    ///
    /// ## Note
    /// duplicate observables are not taken into account. Duplicate Observables should
    /// be romoved at parse stage
    pub fn to_token_stream(&self, observables: &'a Observables) -> TokenStream2 {
        let ObserverBuilder {
            pre_add_observer_tokens,
            add_observer_body_tokens: add_observer_body,
            borrow,
        } = self;

        assert!(!observables.is_empty());

        let mut buff = TokenStream2::new();

        let borrow_buff = if *borrow {
            observables.borrowed_token_stream()
        } else {
            TokenStream2::new()
        };

        let (observable_name, value_name) = if observables.len() > 1 {
            buff = quote! {
                let __multi_observer = State::from(());
            };
            // add ownership of observables to multi_observer state
            for observable in observables.iter() {
                buff.append_all(quote! {
                    add_multi_observer(&#observable, std::rc::Rc::downgrade(&__multi_observer));
                });
            }
            // clone observables
            for observable in observables.iter() {
                buff.append_all(quote! {
                    let #observable = #observable.clone();
                });
            }

            (quote! {__multi_observer}, quote! {_})
        } else {
            (observables[0].clone(), observables[0].clone())
        };

        quote! {{
            let __node = std::rc::Rc::downgrade(&__node);
            #buff
            #pre_add_observer_tokens
            #observable_name.add_observer(Box::new(move |#value_name| {
                if let Some(__node) = __node.upgrade() {
                    #borrow_buff
                    #add_observer_body
                    false
                } else {
                    true
                }
            }));
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::{Observables, ObserverBuilder};
    use quote::quote;

    #[test]
    fn test() {
        // WARN: write tests
        let builder = ObserverBuilder {
            pre_add_observer_tokens: &quote! {},
            add_observer_body_tokens: &quote! {},
            borrow: false,
        }
        .to_token_stream(&Observables(vec![quote! {hello, hi}]));
    }
}
