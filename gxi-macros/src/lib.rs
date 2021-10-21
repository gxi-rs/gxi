use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
mod comp;
mod state;

#[proc_macro]
pub fn set_state(input: TokenStream) -> TokenStream {
    let state::StateParser { names, body } = syn::parse_macro_input!(input as state::StateParser);

    let mut clone_tt = TokenStream2::new();
    let mut borrow_tt = TokenStream2::new();
    let mut notify_tt = TokenStream2::new();

    for name in &names.elems {
        let name_under = syn::Ident::new(&format!("_{}", quote! {#name})[..], name.span());
        clone_tt.append_all(quote! {
            let #name_under = #name.clone();
        });
        borrow_tt.append_all(quote! {
            let mut #name = (*#name_under).borrow_mut();
        });
        notify_tt.append_all(quote! {
            #name_under.notify();
        });
    }

    let exec_block = if let syn::Expr::Async(syn::ExprAsync { block, .. }) = body {
        if cfg!(feature = "web") {
            let mut clone_tt2 = TokenStream2::new();
            for name in &names.elems {
                let name_under = syn::Ident::new(&format!("_{}", quote! {#name})[..], name.span());
                clone_tt2.append_all(quote! {
                    let #name = #name_under.clone();
                });
            }
            quote! {
                #clone_tt2
                gxi::spawn_local(async move {
                    {
                        #block
                    };
                    #notify_tt
                });
            }
        } else {
            panic!("async state not supported for current feature flag");
        }
    } else {
        quote! {
            {
                #borrow_tt
                #body
            };
            #notify_tt
        }
    };

    (quote! {
        {
            #clone_tt
            move |e| {
                #exec_block
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
        viz,
    } = syn::parse_macro_input!(input as comp::CompParser);

    (quote! {
        #viz struct #name (gxi::StrongNodeType);

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
