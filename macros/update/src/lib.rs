use proc_macro::TokenStream;

use quote::quote;
use syn::__private::*;

///
/// `Derive macro` for generating the `update` function for the component.
/// ## Syntax
///
/// ```rust
/// #[update(NameOfComponent)]
/// async fn update<F: Fn() + 'static>(state: AsyncState, msg: Msg, _render: F) -> AsyncResult<ShouldRender> {
///     // --update-logic--
/// }
/// ```
///
/// ## Use
///
/// > This is same as the `update` block of the [comp macro](../comp/macro.comp.html#update-block)
///
/// Syntax highlighting on most IDE's and text editors for proc-macros in rust is very poor.
/// Therefore to resolve this issue this macro allows you to write a function wrapped in this macro.
/// This allows the IDE to provide proper syntax highlighting and code completion for the business logic.
#[proc_macro_attribute]
pub fn update(name: TokenStream, item: TokenStream) -> TokenStream {
    let update_fn = syn::parse_macro_input!(item as syn::ItemFn);
    //check if update_fn has the name update
    if update_fn.sig.ident.to_string() != "update" {
        panic!("Function must be named update");
    }

    let name = syn::parse_macro_input!(name as syn::Ident);

    let update_inner = if cfg!(feature = "desktop") {
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
                //update logic. Made to return should render to force dev to decide render state
                #update_fn
                if let ShouldRender::Yes = update(state,msg,render).await.unwrap() {
                    channel_sender.send(()).unwrap()
                }
            });
        }
    } else {
        quote! {
            let state = {
                let state_borrow = this.as_ref().borrow();
                let state = state_borrow.as_any().downcast_ref::<#name>().unwrap();
                state.state.clone()
            };
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
                //update logic. Made to return should render to force dev to decide render state
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

    (quote! {
        impl #name {
            fn update(this: NodeRc, msg: Msg) {
                #update_inner
            }
        }
    })
    .into()
}
