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

                fn as_node(&self) -> gxi::Node {
                    gxi::Node::#node
                }
            }

            //impl AsRef<dyn std::any::Any> for #name {
            //    fn as_ref(&self) -> &dyn std::any::Any {
            //        self
            //    }
            //}
            //impl AsRef<dyn #trait_tok> for #name {
            //    fn as_ref(&self) -> &dyn #trait_tok {
            //        self
            //    }
            //}

            //impl AsMut<dyn #trait_tok> for #name {
            //    fn as_mut(&mut self) -> &mut dyn #trait_tok {
            //        self
            //    }
            //}

            //impl AsRef<dyn std::any::Any> for #name {
            //    fn as_ref(&self) -> &dyn std::any::Any {
            //        self
            //    }
            //}

            //impl AsMut<dyn std::any::Any> for #name {
            //    fn as_mut(&mut self) -> &mut dyn std::any::Any {
            //        self
            //    }
            //}
        }
    }
}
