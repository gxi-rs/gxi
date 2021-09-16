use proc_macro::TokenStream;

use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;

use crate::derive_vnode::derive_vnode;
use crate::v_node_type::VNodeType;

pub fn parse_component_derive(input: TokenStream) -> syn::Result<TokenStream2> {
    let input = syn::parse::<syn::DeriveInput>(input).unwrap();
    let name = &input.ident;

    return if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = fields {
            let mut init_fields = TokenStream2::new();

            for field in named.iter() {
                let ident = &field.ident;

                if ident.as_ref().unwrap().to_string() == "node" {
                    init_fields.append_all(quote! {
                         node : gxi::ContainerNode {
                             parent,
                             self_substitute: None,
                             child: Default::default(),
                             sibling: Default::default(),
                         },
                    })
                } else {
                    init_fields.append_all(quote! {
                        #ident : Default::default(),
                    })
                }
            }

            let v_node_impl = derive_vnode(
                name,
                VNodeType::Component,
                &quote! {
                    Self {
                        #init_fields
                    }
                },
            );

            Ok(quote! {
                impl gxi::VComponent for #name {
                    fn get_node(&self) -> &gxi::ContainerNode {
                        &self.node
                    }
                    fn get_node_mut(&mut self) -> &mut gxi::ContainerNode {
                        &mut self.node
                    }
                }
                #v_node_impl
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "#[derive(gxi::Component)] can only be used on structs with named fields",
            ))
        }
    } else {
        Err(syn::Error::new(
            input.span(),
            "#[derive(gxi::Component)] can only be used on structs",
        ))
    };
}
