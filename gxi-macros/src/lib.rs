use proc_macro::TokenStream;
use quote::{ToTokens, __private::Span, quote, spanned::Spanned};

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

#[proc_macro_attribute]
pub fn comp(attrs: TokenStream, input: TokenStream) -> TokenStream {
    if !attrs.is_empty() {
        return syn::Error::new(
            syn::parse::<syn::Ident>(attrs).unwrap().span(),
            "didn't expect this here",
        )
        .to_compile_error()
        .into();
    }

    let syn::ItemStruct {
        ident,
        fields,
        vis,
        generics,
        attrs,
        ..
    } = syn::parse_macro_input!(input as syn::ItemStruct);

    if !generics.to_token_stream().is_empty() {
        return syn::Error::new(generics.__span(), "gxi::Component can't be generic")
            .to_compile_error()
            .into();
    }

    let state_ty = syn::Ident::new(&format!("{}State", &ident)[..], Span::call_site());
    
    let fields = fields.iter();

    (quote! {
        #[derive(gxi::Component)]
        #vis struct #ident {
            #vis node: gxi::ContainerNode,
            #vis state: gxi::State<#state_ty>,
        }

        #[derive(Default)]
        #(#attrs)*
        #vis struct #state_ty { 
            #(#vis #fields),*
        }
    })
    .into()
}
