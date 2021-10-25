use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_widget_derive(input: TokenStream) -> TokenStream2 {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &input.ident;

    let v_node_impl = derive_vnode(name, VNodeType::Widget);

    quote! {
        impl gxi::VWidget for #name {
        }

        impl std::ops::Deref for #name {
            type Target = gxi::NativeWidget;

            fn deref(&self) -> &Self::Target {
                &self.native_widget
            }
        }

        impl std::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.native_widget
            }
        }

        #v_node_impl
    }
}
