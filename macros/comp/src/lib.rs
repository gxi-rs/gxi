use proc_macro::TokenStream;

use parsers::comp_parser::CompParser;

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
#[proc_macro]
pub fn comp(item: TokenStream) -> TokenStream {
    let CompParser { tree } = syn::parse_macro_input!(item as CompParser);
    tree.into()
}
