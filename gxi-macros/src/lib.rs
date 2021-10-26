use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;

mod comp;
mod set_state;

/// manages state ownership, borrow, and async event handlers to reduce boiler plate code
///
/// ## args
///     
/// 1. `expression`
///     + if expression is of type closure, then automatic borrow is avoided
///     + otherwise dependent variables are cloned and borrowed to scope.
///     
///     *e.g*
///     
///     ```ignore
///         set_state!(*counter+=1);
///     ```
///     
///     *is equal to*
///     
///     ```ignore
///         set_state!(|_| {
///             *counter+=1;
///         }, [ref counter])
///     ```
///     
///     *or*
///
///     ```ignore
///         set_state!(|_| {
///             *counter.as_ref().borrow_mut()+=1;
///         }, [counter])
///     ```
///     
/// 2. `variables` on which closure depends on
///
/// In case variables not are specified then dependencies are automatically guessed.
/// If first expression is of type closure then automatic guessing is avoided, due to complexity of
/// closure expressions.
///
/// *e.g*
/// ```ignore
/// pub fn app() -> StrongNodeType {
///     let counter = gxi::State::new(2);
///     
///     let click_handler = set_state!(|_| {
///         *counter += 1;
///     }, [ref counter]);
///
///     return gxi! {
///         div [
///             h1 ( const on_click = click_handler, inner_html = &counter.to_string()[..] ),
///             button ( on_click = set_state!(counter += 1) )
///         ]
///     }
/// }
///
/// ```
/// *move* is automatically added to resultant closure regardless of expression type
///
/// ## dependency prefix
///
/// passed dependencies can be prefixed with keywords to reduce boiler plate code
///
/// + `ref` => <attr>.as_ref().borrow_mut()
#[proc_macro]
pub fn set_state(input: TokenStream) -> TokenStream {
    let mod_state = syn::parse_macro_input!(input as set_state::SetState);
    mod_state.to_token_stream().into()
}

#[proc_macro_attribute]
pub fn comp(_: TokenStream, input: TokenStream) -> TokenStream {
    let comp::CompParser {
        name,
        render_func,
        new_func,
        viz,
    } = syn::parse_macro_input!(input as comp::CompParser);

    (quote! {
        #viz struct #name (gxi::StrongNodeType);

        impl #name {
            #new_func
            #[allow(non_snake_case)]
            #render_func
            pub fn into_strong_node_type(self) -> gxi::StrongNodeType {
                self.0
            }
        }
    })
    .into()
}
