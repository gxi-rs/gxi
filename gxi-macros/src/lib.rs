use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn update(input: TokenStream) -> TokenStream {
    let expr = syn::parse_macro_input!(input as syn::Expr);
    (quote! {
        {
            let __weak_this = Rc::downgrade(&this);
            move |_| Self::update(&__weak_this, #expr)
        }
    })
    .into()
}
