use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::{parse::Parse, spanned::Spanned};

pub struct SetState {
    closure: syn::ExprClosure,
    dependency_array: DependencyArray,
}

impl Parse for SetState {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse expression
        let expr = input.parse::<syn::Expr>()?;
        let closure = if let syn::Expr::Closure(closure) = expr {
            closure
        } else {
            syn::parse(
                quote! {
                    |_| {
                       #expr
                    }
                }
                .into(),
            )?
        };

        if !input.is_empty() {
            input.parse::<syn::Token!(,)>()?;
        }
        // parse dependency array
        Ok(Self {
            closure,
            dependency_array: input.parse::<DependencyArray>()?,
        })
    }
}

impl ToTokens for SetState {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let Self {
            closure,
            dependency_array,
        } = self;

        let syn::ExprClosure {
            attrs,
            asyncness,
            inputs,
            output,
            body,
            ..
        } = closure;

        let mut clone_tt = TokenStream2::new();
        let mut extra_tt = TokenStream2::new();
        let mut notify_tt = TokenStream2::new();

        for ele in &dependency_array.0 {
            let name = ele.get_ident();
            clone_tt.append_all(quote! {let #name = #name.clone();});
            extra_tt.append_all(quote! {#ele});
            notify_tt.append_all(quote! {#name.notify();})
        }

        let mut body = quote! {
            {
                #extra_tt
                #body
            }
            #notify_tt
        };

        if let Some(asyncness) = asyncness {
            if cfg!(feature = "web") {
                body = quote! {
                    #clone_tt
                    gxi::spawn_local(#asyncness move {
                        #body
                    });
                };
            } else {
                panic!(
                    "{}",
                    syn::Error::new(
                        asyncness.span(),
                        "async not yet supported for current feature flag",
                    )
                    .to_string()
                );
            }
        }

        tokens.append_all(quote! {
            {
                #clone_tt
                #(#attrs)*
                move |#inputs| #output {
                    #body
                }
            }
        });
    }
}

/// attributes of `mod_state` macro.
///
/// *valid prefixes*
/// + ref
pub enum Dependency {
    Ref(syn::Ident),
    Normal(syn::Ident),
}

impl Parse for Dependency {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.parse::<syn::Token!(ref)>().is_ok() {
            Ok(Self::Ref(input.parse()?))
        } else {
            Ok(Self::Normal(input.parse()?))
        }
    }
}

impl Dependency {
    fn get_ident(&self) -> &syn::Ident {
        match self {
            Dependency::Ref(ident) => ident,
            Dependency::Normal(ident) => ident,
        }
    }
}

impl ToTokens for Dependency {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        if let Dependency::Ref(name) = self {
            tokens.append_all(quote! {let #name = &mut *(**#name).borrow_mut();})
        }
    }
}

#[derive(Default)]
pub struct DependencyArray(Vec<Dependency>);

impl Parse for DependencyArray {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut dependency_array = Vec::new();
        let bracket = syn::__private::parse_brackets(input)?;
        let content = bracket.content;

        while !content.is_empty() {
            dependency_array.push(content.parse()?);
            if !content.is_empty() {
                content.parse::<syn::Token!(,)>()?;
            }
        }

        Ok(Self(dependency_array))
    }
}
