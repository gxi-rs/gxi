use proc_macro::TokenStream;

use quote::quote;
use syn::__private::TokenStream2;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_top_level_container_derive(input: TokenStream) -> TokenStream2 {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &input.ident;
    
    let v_node_impl = derive_vnode(name, VNodeType::TopLevelContainer, & quote! {
        Self::default()
    });

    quote! {
        impl gxi::VTopLevelContainer for #name {
            fn get_node(&self) -> &gxi::TopLevelNode {             
                &self.node
            }
                                                                     
            fn get_node_mut(&mut self) -> &mut gxi::TopLevelNode {
                &mut self.node
            }
        }
        
        #v_node_impl
    }
}
