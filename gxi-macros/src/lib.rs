use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
mod comp;
mod state;

#[proc_macro]
pub fn set_state(input: TokenStream) -> TokenStream {
    let state::StateParser { names, body } = syn::parse_macro_input!(input as state::StateParser);

    let mut clone_tt = TokenStream2::new();
    let mut borrow_tt = TokenStream2::new();
    let mut notify_tt = TokenStream2::new();

    for name in &names.elems {
        clone_tt.append_all(quote! {
            let mut #name = #name.clone();
        });
        borrow_tt.append_all(quote! {
            let #name = &mut *(*#name).borrow_mut();
        });
        notify_tt.append_all(quote! {
            #name.notify();
        });
    }

    (quote! {
        {
            #clone_tt
            move |e| {
                {
                    #borrow_tt
                    #body
                };
                #notify_tt
            }
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
