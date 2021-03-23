use proc_macro::TokenStream;

#[proc_macro]
pub fn proc_cont(item: TokenStream) -> TokenStream {
  //  let input = syn::parse_macro_input!(item as syn::ItemFn);
    item
}
