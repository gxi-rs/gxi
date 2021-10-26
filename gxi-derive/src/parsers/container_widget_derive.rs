use proc_macro::TokenStream;

use quote::{ToTokens, quote};
use syn::__private::TokenStream2;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_container_widget_derive(input: TokenStream) -> TokenStream2 {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = input.ident.to_token_stream();

    let v_node_impl = derive_vnode(&name, &VNodeType::ContainerWidget.into_token_stream());

    quote! {
        impl gxi::VContainerWidget for #name {
            fn get_children(&self) -> &Vec<gxi::StrongNodeType> {
                &self.children
            }
            fn get_children_mut(&mut self) -> &mut Vec<gxi::StrongNodeType> {
                &mut self.children
            }
        }

        impl std::ops::Deref for #name {
            type Target = gxi::NativeContainerWidget;

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
