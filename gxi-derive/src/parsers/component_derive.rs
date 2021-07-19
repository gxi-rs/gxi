use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_component_derive(input: TokenStream) -> TokenStream2 {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &input.ident;
    let v_node_impl = derive_vnode(name, VNodeType::Component);

    quote! {
        impl gxi::VComponent for #name {
            fn get_node_ref(&self) -> &std::cell::RefCell<gxi::ContainerNode> {
                self.node.as_ref()
            }
        }
        #v_node_impl
    }
}
