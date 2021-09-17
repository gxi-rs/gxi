use quote::{quote, ToTokens, TokenStreamExt};
use syn::{__private::TokenStream2, parse::Parse, spanned::Spanned};

pub struct UpdateParser(TokenStream2);

impl Parse for UpdateParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let syn::ItemFn {
            block, sig, vis, ..
        } = input.parse::<syn::ItemFn>()?;

        // check for args
        let mut extra = TokenStream2::new();

        {
            let args = &sig.inputs;
            let mut has_msg_arg = false;

            for ele in args {
                if let syn::FnArg::Typed(arg) = ele {
                    let pat = &arg.pat;
                    match &pat.to_token_stream().to_string()[..] {
                        "state" => {
                            let ty = &arg.ty;

                            extra.append_all(quote! {
                            let state:#ty = &mut __this_borrow.as_mut().downcast_mut::<Self>().unwrap().state;
                            })
                        }
                        "msg" => has_msg_arg = true,
                        _ => {
                            return Err(syn::Error::new(arg.span(), "didn't expect this arg here"));
                        }
                    }
                } else {
                    return Err(syn::Error::new(ele.span(), "expected typed arg"));
                }
            }

            if !has_msg_arg {
                return Err(syn::Error::new(args.span(), "expected msg arg"));
            }
            if !extra.is_empty() {
                extra = quote! {
                    let mut __this_borrow = this.as_ref().borrow_mut();
                    #extra
                };
            }
        }

        Ok(Self(quote! {
            #vis fn update(__this: &gxi::WeakNodeType, msg: Self::Msg) {
                if let Some(this) = __this.upgrade() {
                    use gxi::Renderable;

                    {
                        #extra
                        #block
                    }

                    Self::render(&this)
                }
            }
        }))
    }
}

impl ToTokens for UpdateParser {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let t = &self.0;
        tokens.append_all(quote! { #t });
    }
}
