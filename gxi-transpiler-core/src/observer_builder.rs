use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;

use crate::snippets::{self, AddObserverCondition};
use crate::state::State;

pub struct ObserverBuilder<'a> {
    pub pre_add_observer_tokens: &'a TokenStream2,
    pub body: &'a TokenStream2,
    pub post_add_observer_tokens: &'a TokenStream2,
    pub state: &'a State,
    /// if true and one observable
    /// then `.borrow()` is called on the `RefCell` closure value
    /// else if true and more than one observables
    /// then dont borrow
    pub borrow: bool,
}

impl<'a> ToTokens for ObserverBuilder<'a> {
    /// if there is only one observable
    /// add_observer is called on the observable with a closure `|RefCell|`
    ///
    /// ## Args
    /// * `observables` - a non empty array of identifiers whose value has to observed
    ///
    /// ## Note
    /// duplicate observables are not taken into account. Duplicate Observables should
    /// be romoved at parse stage
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let ObserverBuilder {
            pre_add_observer_tokens,
            body,
            post_add_observer_tokens,
            state,
            borrow,
        } = self;

        let observables = if let State::Observable(observables) = state {
            observables
        } else {
            body.to_tokens(tokens);
            return;
        };

        let mut multi_observable_tokens = TokenStream2::new();

        let (observable_name, value_name) = if observables.len() > 1 {
            snippets::ObservableAction::NewMultiObserver.to_tokens(&mut multi_observable_tokens);
            // add ownership of observables to multi_observer state
            for observable in observables.iter() {
                snippets::ObservableAction::AddMultiObserverTo(observable)
                    .to_tokens(&mut multi_observable_tokens);
            }
            // clone observables
            for observable in observables.iter() {
                snippets::StdAction::Clone(snippets::VariableName::Other(observable))
                    .to_tokens(&mut multi_observable_tokens);
            }

            (
                snippets::VariableName::MultiObserver.to_token_stream(),
                snippets::VariableName::None.to_token_stream(),
            )
        } else {
            (observables[0].clone(), observables[0].clone())
        };

        let body = {
            let mut body = if *borrow {
                observables.borrowed_token_stream(&(0..observables.len()))
            } else {
                TokenStream2::new()
            };

            self.body.to_tokens(&mut body);

            body
        };

        let add_observer = snippets::StateAction::AddObserver {
            value_name: &value_name,
            observable_name: &observable_name,
            observer_condition: match state {
                State::Constant => AddObserverCondition::Node(&body),
                // FIX: NodeWithCtx
                State::Observable(_) => AddObserverCondition::Node(&body),
            },
        };

        tokens.append_all({
            let mut tokens = TokenStream2::new();

            snippets::RcAction::Downgrade(&snippets::VariableName::Node.to_token_stream())
                .to_tokens(&mut tokens);

            multi_observable_tokens.to_tokens(&mut tokens);
            pre_add_observer_tokens.to_tokens(&mut tokens);
            add_observer.to_tokens(&mut tokens);
            post_add_observer_tokens.to_tokens(&mut tokens);

            quote!({#tokens})
        });
    }
}
