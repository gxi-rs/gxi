use proc_macro::TokenStream;
use quote::quote;

mod comp;
mod state;

#[proc_macro]
pub fn set_state(input: TokenStream) -> TokenStream {
    let state::StateParser { name, body } = syn::parse_macro_input!(input as state::StateParser);

    (quote! {
        {
            let mut #name = #name.clone();
            move |e| #name.set_value(#body)
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn comp(_: TokenStream, input: TokenStream) -> TokenStream {
    let comp::CompParser {
        name,
        render_func,
        new_func,
    } = syn::parse_macro_input!(input as comp::CompParser);

    (quote! {
        struct #name (gxi::StrongNodeType);

        impl #name {
            #new_func
            #[allow(non_snake_case)]
            #render_func
            pub fn into_strong_node_type(self) -> gxi::StrongNodeType {
                self.0
            }
        }
    })
    .into()
}
