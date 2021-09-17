use proc_macro::TokenStream;
use quote::quote;

mod render;
mod update;

#[proc_macro]
pub fn set_state(input: TokenStream) -> TokenStream {
    let expr = syn::parse_macro_input!(input as syn::Expr);
    (quote! {
        {
            use gxi::StatefulNode;

            let __weak_this = std::rc::Rc::downgrade(&this);
            move |_| Self::update(&__weak_this, #expr)
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn render(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let comp_name = syn::parse_macro_input!(attrs as syn::Ident);
    let render_fn = syn::parse_macro_input!(input as render::RenderParser);

    (quote! {
        impl gxi::Renderable for #comp_name {
            #render_fn
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn update(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let comp_name = syn::parse_macro_input!(attrs as syn::Ident);
    let update_fn = syn::parse_macro_input!(input as update::UpdateParser);

    let state_ty = syn::Ident::new(&format!("{}State", &comp_name)[..], comp_name.span());

    //TODO: parse optional Msg name from attrs

    (quote! {
        impl gxi::StatefulNode for #comp_name {
            type Msg = Msg;
            type State = gxi::State<#state_ty>;

            #update_fn
        }
    })
    .into()
}
