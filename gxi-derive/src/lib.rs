extern crate proc_macro;

use proc_macro::TokenStream;

use crate::parsers::*;

mod derive_vnode;
mod parsers;
mod v_node_type;

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

