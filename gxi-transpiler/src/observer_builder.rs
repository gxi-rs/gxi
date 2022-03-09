use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;

pub struct ObserverBuilder<'a> {
    pub pre_add_observer_tokens: &'a TokenStream2,
    pub add_observer_body_tokens: &'a TokenStream2,
}

impl<'a> ObserverBuilder<'a> {
    pub fn to_token_stream(&self, observables: &'a Vec<TokenStream2>) -> TokenStream2 {
        let ObserverBuilder {
            pre_add_observer_tokens,
            add_observer_body_tokens: add_observer_body,
        } = self;

        let mut buff = TokenStream2::new();

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
                    #add_observer_body
                    false
                } else {
                    true
                }
            }));
        }}
    }
}
