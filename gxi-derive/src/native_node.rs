use quote::quote;
use syn::__private::TokenStream2;

pub enum NativeNode {
    Container,
    Widget,
}

impl NativeNode {
    pub fn impl_deref(&self, name: &TokenStream2) {
        let target = match self {
            Self::Container => quote!(gxi::NativeContainer),
            Self::Widget => quote!(gxi::NativeWidget),
        };

        quote! {
            impl std::ops::Deref for #name {
                 type Target = #target;

                 fn deref(&self) -> &Self::Target {
                     &self.native_widget
                 }
             }

             impl std::ops::DerefMut for #name {
                 fn deref_mut(&mut self) -> &mut Self::Target {
                     &mut self.native_widget
                 }
             }
        };
    }
}
