use quote::quote;
use syn::__private::TokenStream2;

pub fn derive_vnode(name: &TokenStream2, v_node_type: &TokenStream2) -> TokenStream2 {
    quote! {
        impl gxi::VNode for #name {
            fn new() -> Self {
                Self::default()
            }

            fn into_strong_node_type(self) -> gxi::StrongNodeType {
                std::rc::Rc::new(std::cell::RefCell::new(
                    gxi::VNodeType::#v_node_type(Box::from(self)
                )))
            }
        }

        impl AsRef<dyn gxi::VNode> for #name {
            fn as_ref(&self) -> &dyn gxi::VNode {
                self
            }
        }

        impl AsMut<dyn gxi::VNode> for #name {
            fn as_mut(&mut self) -> &mut dyn gxi::VNode {
                self
            }
        }

        impl AsRef<dyn std::any::Any> for #name {
            fn as_ref(&self) -> &dyn std::any::Any {
                self
            }
        }

        impl AsMut<dyn std::any::Any> for #name {
            fn as_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    }
}
