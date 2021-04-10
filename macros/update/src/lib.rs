use proc_macro::TokenStream;

use quote::quote;
use syn::__private::*;

///
/// ## Format
///
/// ```rust
///   comp! (
///       MyApp { //name of component
///           count : u32 = 0 // property : type = default value (optional)
///       }
///       update { //update function
///
///       }
///       render { //render function
///           Component
///       }
///   );
/// ```
#[proc_macro_attribute]
pub fn update(name: TokenStream, item: TokenStream) -> TokenStream {
    let update_fn = syn::parse_macro_input!(item as syn::ItemFn);
    //check if update_fn has the name update
    if update_fn.sig.ident.to_string() != "update" {
        panic!("Function must be named update");
    }

    let name = syn::parse_macro_input!(name as syn::Ident);
    let state_name = syn::Ident::new(&format!("{}State", quote!(#name)), Span::call_site());

    (quote! {
        type State = Arc<Mutex<#state_name>>;

        impl #name {

            fn update(this: NodeRc, msg: Msg) {
                //the channel logic can be abstracted away to be platform specific
                let (channel_sender, state) = {
                    let state_borrow = this.as_ref().borrow();
                    let state = state_borrow.as_any().downcast_ref::<#name>().unwrap();
                    (state.channel_sender.clone(), state.state.clone())
                };
                task::spawn(async move {
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
        }
    }).into()
}