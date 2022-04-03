use gxi_transpiler_core::RootBlock;
use proc_macro::TokenStream;
use quote::ToTokens;

/// Refer to [`gxi_transpiler_core`](mod@gxi_transpiler_core)
#[proc_macro]
pub fn gxi(input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(input as RootBlock)
        .to_token_stream()
        .into()
}
