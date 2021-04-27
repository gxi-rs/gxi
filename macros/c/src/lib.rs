use proc_macro::TokenStream;

use parsers::TreeParser;

/// This macro is used to write the component tree. It is recommended to not use the macro manually.
/// ## Syntax
///
/// ```rust
/// c! (
///    pure_index init_type component_name ( properties ) [
///         children,
///         { /* Code can be executed here on every render */ },
///    ]
/// )
/// ```
///
/// * `index` - The index of pure component in an if block. Default value is 0
///
/// * `init_type` - The init_type of the component. Possible values : \[ init_child/init_sibling \]
///
/// * `component_name` - The name of the component.
///
/// * `properties` - `,` separated properties in the form of [ExprAssign](syn::ExprAssign).
///
/// * `children` - `,` separated children recursive to the macro itself.
///
/// ## Conditional Rendering
///
/// ```rust
/// c! (
///     if 3 == 3
///         for i in 0..3
///             Text ( label = &i.to_string()[..] )
///     else
///         Button
/// )
/// ```
///
/// The next statement after if block can be a `for` statement or an if statement or a component.
/// After which an optional else block can be used following the same syntax as above.
///
/// ## Looping
///
/// ```rust
/// c! (
///    for i in 0..3
///         if i == 1
///             Text ( label = &i.to_string()[..] )
/// )
/// ```
///
/// The syntax scheme as if can be used here.
///
/// Both `if/else` and `for` expect only 1 statement after them. It can be a `Component` or other `if/else` or a for statement.
///
#[proc_macro]
pub fn c(item: TokenStream) -> TokenStream {
    let TreeParser { tree } = syn::parse_macro_input!(item as TreeParser);
    tree.into()
}
