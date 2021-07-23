use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_component_derive(input: TokenStream) -> TokenStream2 {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &input.ident;


    let v_node_impl = derive_vnode(name, VNodeType::Component, &quote! {
        Self {
            node : gxi::ContainerNode {
                parent,
                child: Default::default(),
                sibling: Default::default(),
            },
        }
    });

    quote! {
        impl gxi::VComponent for #name {
            fn get_node(&self) -> &gxi::ContainerNode {             
                &self.node
            }
            fn get_node_mut(&mut self) -> &mut gxi::ContainerNode {
                &mut self.node
            }
        }
        #v_node_impl
    }
}
