use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::{parse::Parse, spanned::Spanned};
pub struct ModState {
    closure: syn::ExprClosure,
    expr_attr: Option<Vec<syn::Attribute>>,
    expr_left: Option<Box<syn::Expr>>,
    expr_eq: Option<syn::Token!(=)>,
}

impl Parse for ModState {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(syn::ExprAssign {
            right,
            attrs,
            left,
            eq_token,
        }) = input.parse::<syn::ExprAssign>()
        {
            if let syn::Expr::Closure(closure) = *right {
                Ok(Self {
                    closure,
                    expr_attr: Some(attrs),
                    expr_eq: Some(eq_token),
                    expr_left: Some(left),
                })
            } else {
                Err(syn::Error::new(right.span(), "expected closure"))
            }
        } else {
            Ok(Self {
                closure: input.parse()?,
                expr_attr: None,
                expr_eq: None,
                expr_left: None,
            })
        }
    }
}

impl ModState {
    pub fn to_tokens(&self, attr: &ModStateAttrs) -> syn::Result<TokenStream2> {
        let Self {
            closure,
            expr_attr,
            expr_left,
            expr_eq,
        } = self;

        let closure = {
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

            for ele in &attr.0 {
                let name = ele.get_ident();
                clone_tt.append_all(quote! {#name});
                extra_tt.append_all(quote! {#ele});
            }

            let mut body = quote! {
                #extra_tt
                #body
            };

            if let Some(asyncness) = asyncness {
                if cfg!(feature = "web") {
                    body = quote! {
                        gxi::span_local(#asyncness {
                            #clone_tt
                            #body
                        });
                    };
                } else {
                    return Err(syn::Error::new(
                        asyncness.span(),
                        "async not yet supported for current feature flag",
                    ));
                }
            }

            quote! {
                {
                    #clone_tt
                    #(#attrs)*
                    move |#inputs| #output {
                        #body
                    }
                }
            }
        };

        let expr_attr = if let Some(v) = expr_attr {
            quote! {#(#v)*}
        } else {
            TokenStream2::new()
        };

        let expr_left = if let Some(v) = expr_left {
            quote! {#v}
        } else {
            TokenStream2::new()
        };

        let expr_eq = if let Some(v) = expr_eq {
            quote! {#v}
        } else {
            TokenStream2::new()
        };

        Ok(quote! {
            #expr_attr
            #expr_left #expr_eq #closure
        })
    }
}

/// attributes of `mod_state` macro.
///
/// *valid prefixes*
/// + ref
pub enum ModStateAttr {
    Ref(syn::Ident),
    Normal(syn::Ident),
}

impl Parse for ModStateAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(_) = input.parse::<syn::Token!(ref)>() {
            Ok(Self::Ref(input.parse()?))
        } else {
            Ok(Self::Normal(input.parse()?))
        }
    }
}

impl ModStateAttr {
    fn get_ident(&self) -> &syn::Ident {
        match self {
            ModStateAttr::Ref(ident) => ident,
            ModStateAttr::Normal(ident) => ident,
        }
    }
}

impl ToTokens for ModStateAttr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        match self {
            ModStateAttr::Ref(name) => tokens.append_all(quote! { #name.as_ref().borrow_mut() }),
            ModStateAttr::Normal(name) => tokens.append_all(quote! {#name}),
        }
    }
}

#[derive(Default)]
pub struct ModStateAttrs(Vec<ModStateAttr>);

impl Parse for ModStateAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attrs = Vec::new();
        while input.is_empty() {
            attrs.push(ModStateAttr::parse(input)?);
            if !input.is_empty() {
                input.parse::<syn::Token!(,)>()?;
            }
        }

        Ok(Self(attrs))
    }
}
