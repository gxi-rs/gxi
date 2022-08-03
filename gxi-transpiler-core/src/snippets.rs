use std::ops::Deref;

use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;

pub enum IndexedContext {
    New,
    SetValue,
    ComputeIndex(usize, usize, usize),
}

impl ToTokens for IndexedContext {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        use VariableName::{Child, Ctx, IndexBuff};

        match self {
            Self::New => tokens.append_all(quote! {
                let mut #Ctx = gxi::IndexedContext::default();
            }),
            Self::SetValue => tokens.append_all(quote! {
                #Ctx.set_value(Box::from(#Child));
            }),
            Self::ComputeIndex(
                index_buff_index,
                dynamic_places_occupied,
                constant_places_occupied,
            ) => tokens.append_all(quote! {
                let (__index, __range_to_remove, __should_replace) =
                    gxi::IndexedContext::compute_index(
                        &mut *(*#IndexBuff).borrow_mut(),
                        #index_buff_index,
                        #dynamic_places_occupied,
                        #constant_places_occupied,
                    );
            }),
        }
    }
}

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

pub enum VNodeActions {
    Push,
    /// VNode.insert_at_index()
    /// usize -> index offset
    InsertAtIndex(usize),
    /// VNode.remove_elements()
    RemoveElements,
}

impl ToTokens for VNodeActions {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        use VariableName::Node;

        match self {
            Self::Push => tokens.append_all(quote! {
                 #Node.push(&__child.as_node(), &*__child);
            }),
            Self::InsertAtIndex(index_offset) => tokens.append_all(quote! {
                #Node.insert_at_index(
                    &__child.as_node(),
                    &*__child,
                    __index + #index_offset,
                    __should_replace,
                );
            }),
            Self::RemoveElements => tokens.append_all(quote! {
                #Node.remove_elements(__range_to_remove);
            }),
        }
    }
}

pub enum DynamicIndex {
    /// Index Buffer to keep track of index of
    /// dynamically generated nodes
    ///
    /// usize -> Number of dynamically allocated blocks (eg. if, for)
    IndexBuff(usize),
}

impl ToTokens for DynamicIndex {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        use VariableName::IndexBuff;
        match self {
            Self::IndexBuff(number_of_dynamically_allocated_blocks) => tokens.append_all(quote! {
                let mut #IndexBuff = std::rc::Rc::new(std::cell::RefCell::new([0usize; #number_of_dynamically_allocated_blocks]));
            }),
        }
    }
}

pub enum StateAction<'a> {
    AddObserver {
        value_name: &'a TokenStream2,
        observable_name: &'a TokenStream2,
        observer_condition: AddObserverCondition<'a>,
    },
}

#[derive(Clone)]
pub enum AddObserverCondition<'a> {
    Node(&'a TokenStream2),
    NodeWithCtx(&'a TokenStream2),
}

impl ToTokens for StateAction<'_> {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            StateAction::AddObserver {
                value_name,
                observable_name,
                observer_condition,
            } => tokens.append_all(quote! {
                #observable_name.add_observer(Box::new(move |#value_name| {
                    #observer_condition
                }));
            }),
        }
    }
}

impl Deref for AddObserverCondition<'_> {
    type Target = TokenStream2;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Node(b) => b,
            Self::NodeWithCtx(b) => b,
        }
    }
}

impl<'a> AddObserverCondition<'a> {
    pub fn swap_body(&mut self, with: &'a TokenStream2) {
        match self {
            Self::Node(ref mut b) => *b = with,
            Self::NodeWithCtx(ref mut b) => *b = with,
        }
    }
}

impl ToTokens for AddObserverCondition<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        use VariableName::{Ctx, Node};

        match self {
            AddObserverCondition::Node(body) => tokens.append_all(quote! {
                if let Some(#Node) = #Node.upgrade() {
                    #body
                    false
                } else {
                    true
                }
            }),
            AddObserverCondition::NodeWithCtx(body) => tokens.append_all(quote! {
                if let (Some(#Node), Some(mut #Ctx)) = (#Node.upgrade(), #Ctx.upgrade()) {
                    #body
                    false
                } else {
                    true
                }
            }),
        }
    }
}

pub enum RcAction<'a> {
    Downgrade(&'a TokenStream2),
}

impl ToTokens for RcAction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Downgrade(name) => tokens.append_all(quote! {
                let #name = std::rc::Rc::downgrade(&#name);
            }),
        }
    }
}

pub enum ObservableAction<'a> {
    NewMultiObserver,
    AddMultiObserverTo(&'a TokenStream2),
}

impl ToTokens for ObservableAction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let multi_observer_tokens = VariableName::MultiObserver;

        match self {
            Self::NewMultiObserver => tokens.append_all(quote! {
                let #multi_observer_tokens = State::from(());
            }),
            Self::AddMultiObserverTo(observable) => tokens.append_all(quote! {
                add_multi_observer(&#observable, std::rc::Rc::downgrade(&#multi_observer_tokens));
            }),
        }
    }
}

pub enum StdAction<'a> {
    Clone(VariableName<'a>),
}

impl ToTokens for StdAction<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Clone(name) => tokens.append_all(quote! {
                let #name = #name.clone();
            }),
        }
    }
}

pub enum VariableName<'a> {
    Node,
    MultiObserver,
    IndexBuff,
    Ctx,
    Child,
    None,
    Other(&'a TokenStream2),
}

impl ToTokens for VariableName<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            Self::Node => tokens.append_all(quote!(__node)),
            Self::MultiObserver => tokens.append_all(quote!(__multi_observer)),
            Self::IndexBuff => tokens.append_all(quote!(__index_buff)),
            Self::Ctx => tokens.append_all(quote!(__ctx)),
            Self::Child => tokens.append_all(quote!(__child)),
            Self::None => tokens.append_all(quote!(_)),
            Self::Other(name) => name.to_tokens(tokens),
        }
    }
}
