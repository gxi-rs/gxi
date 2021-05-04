use proc_macro::TokenStream;

use quote::quote;
use syn::__private::*;
use syn::spanned::Spanned;

/// *derive-macro for generating the update function of a gxi component*
///
/// ## Syntax
///
/// ```rust
/// #[update(NameOfComponent)]
/// async fn update<F: Fn() + 'static>(state: AsyncState, msg: Msg, _render: F) -> AsyncResult<ShouldRender> {
///     // --gxi_update_macro-logic--
/// }
/// ```
///
/// ## Use
///
/// > This is same as the `update` block of the [gxi_macro](../gxi_macro/macro.gxi_macro.html#gxi_update_macro-block)
///
/// Syntax highlighting on most IDE's and text editors for proc-macros in rust is very poor.
/// Therefore to resolve this issue this macro allows you to write a function wrapped in this macro.
/// This allows the IDE to provide proper syntax highlighting and code completion for the business logic.
#[proc_macro_attribute]
pub fn update(name: TokenStream, item: TokenStream) -> TokenStream {
    let update_fn = syn::parse_macro_input!(item as syn::ItemFn);
    //check if update_fn has the name gxi_update_macro
    if update_fn.sig.ident.to_string() != "update" {
        return syn::Error::new(
            update_fn.sig.span(),
            "Function must be named `update`")
            .to_compile_error().into();
    }

    let is_async = update_fn.sig.asyncness.is_some();

    let name = syn::parse_macro_input!(name as syn::Ident);

    let update_inner = {
        let state_cloner = quote! {
            let state = {
                let state_borrow = this.as_ref().borrow();
                let state = state_borrow.as_any().downcast_ref::<#name>().unwrap();
                state.state.clone()
            };
        };
        let update_inner = if !is_async {
            quote! {
                #state_cloner
                let render = {
                    let this = Rc::clone(&this);
                    move || {
                        let this = Rc::clone(&this);
                        {
                            let mut node = this.as_ref().borrow_mut();
                            node.mark_dirty();
                        }
                        Self::render(this);
                    }
                };
                #update_fn
                if let ShouldRender::Yes = update(state,msg,render).unwrap() {
                    {
                        let mut node = this.as_ref().borrow_mut();
                        node.mark_dirty();
                    }
                    Self::render(this);
                }
            }
        } else if cfg!(feature = "desktop") {
            quote! {
                let (channel_sender, state) = {
                    let state_borrow = this.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<#name>().unwrap();
                    (state.channel_sender.clone(), state.state.clone())
                };
                tokio::task::spawn(async move {
                    let render = {
                        let channel_sender = channel_sender.clone();
                        move || channel_sender.send(()).unwrap()
                    };
                    //gxi_update_macro logic. Made to return should render to force dev to decide render state
                    #update_fn
                    if let ShouldRender::Yes = update(state,msg,render).await.unwrap() {
                        channel_sender.send(()).unwrap()
                    }
                });
            }
        } else {
            quote! {
                #state_cloner
                spawn_local(async move {
                    let render =  {
                        let this = Rc::clone(&this);
                        move || {
                            let this = Rc::clone(&this);
                            {
                                let mut node = this.as_ref().borrow_mut();
                                node.mark_dirty();
                            }
                            Self::render(this);
                        }
                    };
                    //gxi_update_macro logic. Made to return should render to force dev to decide render state
                    #update_fn
                    if let ShouldRender::Yes = update(state,msg,render).await.unwrap() {
                        {
                            let mut node = this.as_ref().borrow_mut();
                            node.mark_dirty();
                        }
                        Self::render(this);
                    }
                });
            }
        };
        update_inner
    };
    (quote! {
        impl #name {
            fn update(this: NodeRc, msg: Msg) {
                #update_inner
            }
        }
    }).into()
}
