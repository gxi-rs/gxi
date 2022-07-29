use quote::{quote, ToTokens, TokenStreamExt};

pub enum IndexedContext {
    New,
    SetValue,
}

impl ToTokens for IndexedContext {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            Self::New => tokens.append_all(quote! {
                let mut __ctx = gxi::IndexedContext::default();
            }),
            Self::SetValue => tokens.append_all(quote! {
                __ctx.set_value(Box::from(__child));
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
    /// Vnode.insert_at_index()
    /// usize -> index offset
    InsertAtIndex(usize),
}

impl ToTokens for VNodeActions {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            Self::Push => tokens.append_all(quote! {
                 __node.push(&__child.as_node(), &*__child);
            }),
            Self::InsertAtIndex(index_offset) => tokens.append_all(quote! {
                __node.insert_at_index(
                    &__child.as_node(),
                    &*__child,
                    __index + #index_offset,
                    __should_replace,
                )
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
        match self {
            Self::IndexBuff(number_of_dynamically_allocated_blocks) => {
                tokens.append_all(quote! {
                    let mut __index_buff = std::rc::Rc::new(std::cell::RefCell::new([0usize; #number_of_dynamically_allocated_blocks]));
                })
            },
        }
    }
}
