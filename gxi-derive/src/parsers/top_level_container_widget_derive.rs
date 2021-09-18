use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_top_level_container_widget_derive(input: TokenStream) -> TokenStream2 {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &input.ident;

    let v_node_impl = derive_vnode(name, VNodeType::TopLevelContainerWidget);

    quote! {
        impl gxi::VContainerWidget for #name {
            fn get_node(&self) -> &gxi::TreeNode {
                &self.node
            }

            fn get_node_mut(&mut self) -> &mut gxi::TreeNode {
                &mut self.node
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
