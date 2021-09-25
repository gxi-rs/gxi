use std::collections::HashMap;

use quote::{quote, ToTokens, TokenStreamExt};
use syn::__private::TokenStream2;
use syn::parse::Parse;
use syn::spanned::Spanned;

pub struct CompParser {
    pub name: syn::Ident,
    pub render_func: TokenStream2,
    pub new_func: TokenStream2,
}

fn get_pat_ident(pat: &syn::Pat) -> Option<String> {
    return match pat {
        syn::Pat::Ident(arg) => Some(arg.ident.to_string()),
        syn::Pat::Reference(ref_arg) => get_pat_ident(&*ref_arg.pat),
        syn::Pat::Wild(_) => None,
        _ => unreachable!(),
    };
}
impl Parse for CompParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut func = input.parse::<syn::ItemFn>()?;
        let name = func.sig.ident.clone();
        if name.to_string().chars().next().unwrap().is_lowercase() {
            return Err(syn::Error::new(
                name.span(),
                "should start with an upper case character",
            ));
        }
        if let syn::ReturnType::Default = func.sig.output {
            return Err(syn::Error::new(
                func.sig.span(),
                "expected return type of gxi::StrongNodeType",
            ));
        }

        func.sig.ident = syn::Ident::new("render", name.span());

        let new_func = {
            let args = &func.sig.inputs;

            let mut args_without_type = TokenStream2::new();
            let mut args_with_type = TokenStream2::new();

            // check for duplicate arg ident
            // in case of wild_args(_) replace them with another unique ident
            {
                let mut args_map = HashMap::new();

                for ele in args {
                    if let syn::FnArg::Typed(arg) = ele {
                        if let Some(arg_name) = get_pat_ident(&*arg.pat) {
                            match args_map.entry(arg_name) {
                                std::collections::hash_map::Entry::Vacant(v) => v.insert(()),
                                _ => {
                                    return Err(syn::Error::new(arg.span(), "duplicate"));
                                }
                            };
                        }
                    } else {
                        return Err(syn::Error::new(ele.span(), "didn't expect self here"));
                    }
                }

                {
                    let mut c = 0u32;

                    for ele in args {
                        if let syn::FnArg::Typed(arg) = ele {
                            let name_ident = if let Some(arg_name) = get_pat_ident(&*arg.pat) {
                                syn::Ident::new(&arg_name, arg.span())
                            } else {
                                let mut try_name = format!("__u{}", c);
                                while args_map.contains_key(&try_name) {
                                    c += 1;
                                    try_name = format!("__u{}", c);
                                }
                                let name_ident = syn::Ident::new(&try_name, arg.span());
                                args_map.insert(try_name, ());
                                name_ident
                            };

                            name_ident.to_tokens(&mut args_without_type);
                            name_ident.to_tokens(&mut args_with_type);

                            let ty = &*arg.ty;
                            args_with_type.append_all(quote! {: #ty,});
                            args_without_type.append_all(quote!(,));
                        } else {
                            unreachable!()
                        }
                    }
                }
            }
            quote! {
                pub fn new(#args_with_type) -> Self {
                    Self ( Self::render(#args_without_type) )
                }
            }
        };

        Ok(Self {
            name,
            render_func: func.to_token_stream(),
            new_func,
        })
    }
}
