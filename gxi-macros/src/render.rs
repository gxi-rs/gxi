use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::{parse::Parse, spanned::Spanned};

pub struct RenderParser(TokenStream2);

impl Parse for RenderParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // TODO: check if state.is_dirty()
        let syn::ItemFn {
            block, sig, vis, ..
        } = input.parse::<syn::ItemFn>()?;

        // check for args
        let mut extra = TokenStream2::new();

        {
            let args = &sig.inputs;
            for ele in args {
                if let syn::FnArg::Typed(arg) = ele {
                    match &arg.pat.to_token_stream().to_string()[..] {
                        "state" => {
                            let ty = &arg.ty;
                            extra.append_all(quote! {
                                let state:#ty = &__this_borrow.as_ref().downcast_ref::<Self>().unwrap().state;
                            })
                        }
                        _ => {
                            return Err(syn::Error::new(arg.span(), "didn't expect this arg here"));
                        }
                    }
                } else {
                    return Err(syn::Error::new(ele.span(), "expected typed arg"));
                }
            }

            if !extra.is_empty() {
                extra = quote! {
                    let __this_borrow = this.as_ref().borrow();
                    #extra
                };
            }
        }

        Ok(Self(quote! {
            #vis fn render(this: &gxi::StrongNodeType) {
                #extra
                #block
            }
        }))
    }
}

impl ToTokens for RenderParser {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let t = &self.0;

        tokens.append_all(quote! { #t });
    }
}
