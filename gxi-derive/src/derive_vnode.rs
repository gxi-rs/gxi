use quote::quote;
use syn::__private::TokenStream2;

use crate::v_node_type::VNodeType;

pub fn derive_vnode(
    name: &syn::Ident,
    v_node_type: VNodeType,
    new_impl: &TokenStream2,
) -> TokenStream2 {
    quote! {
        impl gxi::VNode for #name {
            fn new(parent: gxi::WeakNodeType) -> Self {
                #new_impl
            }

            fn into_vnode_type(self) -> gxi::VNodeType {
                gxi::VNodeType::#v_node_type(Box::from(self))
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
