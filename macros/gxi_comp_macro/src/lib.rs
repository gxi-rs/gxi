use proc_macro::TokenStream;

use gxi_parsers::comp_parser::CompParser;

/// This proc-macro is the most efficient and the only recommended way of writing components.
///
/// ## Syntax
///
/// ```rust
/// comp! {
///     NameOfComponent { // Constructor block
///          count : u32 = 0 // property : type = default value ; (optional)
///     }
///     gxi_update_macro { // gxi_update_macro function (Optional)
///         // The business logic goes here
///     }
///     render { // render function (Optional)
///          { /* Component tree goes here */ }
///          Component
///     }
/// };
/// ```
/// ## Render Block
/// The component tree is written in this block. It produces the render function as follows.
///
/// ```rust
///# use std::rc::Rc;
/// fn render(this: NodeRc) {
///    let cont = Rc::clone(&this);
///    let node = cont.clone();
///    let state = {
///        let mut node_borrow = this.as_ref().borrow_mut();
///        let node = node_borrow.as_any_mut().downcast_mut::<Self>().unwrap();
///        if !node.is_dirty() {
///            return;
///        }
///        node.mark_clean();
///        node.state.clone()
///    };
///    let state = state.lock().unwrap();
///    gxi_c_macro!{
///        // the content of the render block is moved here
///    };
/// }
/// ```
///
/// It is recommended not to write the render function manually. Use the render block instead.
/// Synctax for the render block can be found [here](../gxi_c_macro/macro.gxi_c_macro.html#Syntax)
/// ## Update Block
/// *The gxi_update_macro block produces*
///
/// ```rust
/// #[gxi_update_macro(NameOfComponent)]
/// async fn gxi_update_macro<F: Fn() + 'static>(state: AsyncState, msg: Msg, render: F) -> AsyncResult<ShouldRender> {
///     // the content of the gxi_update_macro block is moved here
/// }
/// ```
///
/// In this block values `state: AsyncState, msg: Msg, render: F` are present which can be used as follows ->
///
/// * `state` - is of `type AsyncState = Arc<Mutex<ComponentState>>;` where `ComponentState` has all the fields defined in the
///     constructor block. It must be used to manipulate the state of the component. For efficient renders, the dev should drop the lock
///     on the mutex before awaiting an async call.
///
/// * `msg` - This is the enum defined by the user outside the macro.
///     `Example`
///     ```rust
///     enum Msg {
///         Task
///     }
///     ```
///
/// * `render`- is a closure that can be called to pre-render the component. This can be used in situations where
///     the final value of an async function is not yet received but the user has to be notified about the progress.
///     A popular example is progress bars.
///
#[proc_macro]
pub fn comp(item: TokenStream) -> TokenStream {
    let CompParser { tree } = syn::parse_macro_input!(item as CompParser);
    tree.into()
}
