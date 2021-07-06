extern crate proc_macro;

use proc_macro::TokenStream;

use crate::parsers::{parse_component_derive, parse_widget_derive};

mod derive_vnode;
mod parsers;
mod v_node_type;

#[proc_macro_derive(GxiComponent)]
pub fn gxi_component_derive(input: TokenStream) -> TokenStream {
    parse_component_derive(input).into()
}

#[proc_macro_derive(GxiWidget)]
pub fn gxi_widget_derive(input: TokenStream) -> TokenStream {
    parse_widget_derive(input).into()
}
