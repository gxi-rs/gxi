extern crate proc_macro;

use derive_vnode::derive_vnode;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

use crate::parsers::*;

mod derive_vnode;
mod parsers;
mod v_node_type;

#[proc_macro_attribute]
pub fn gxi_vnode(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse::<syn::ItemStruct>(input).unwrap();
    let name = input.ident.to_token_stream();
    let derive = derive_vnode(&name, &attr.into());
    (quote! {
        #input
        #derive
    })
    .into()
}

#[proc_macro_derive(Widget)]
pub fn gxi_widget_derive(input: TokenStream) -> TokenStream {
    parse_widget_derive(input).into()
}

#[proc_macro_derive(TopLevelContainerWidget)]
pub fn gxi_tpp_level_container_widget_derive(input: TokenStream) -> TokenStream {
    parse_top_level_container_widget_derive(input).into()
}

#[proc_macro_derive(ContainerWidget)]
pub fn gxi_container_widget_derive(input: TokenStream) -> TokenStream {
    parse_container_widget_derive(input).into()
}
