use quote::quote;
use syn::__private::TokenStream2;

pub struct VNodeDefault;

impl VNodeDefault {
    pub fn derive(name: &TokenStream2, node: &TokenStream2) -> TokenStream2 {
        quote! {
            impl gxi::VNode for #name {
                fn new() -> Self {
                    Self::default()
                }

                fn into_node(self) -> gxi::Node {
                    gxi::Node::#node(Box::from(self))
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
}
