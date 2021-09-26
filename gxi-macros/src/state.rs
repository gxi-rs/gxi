use quote::quote;
use syn::parse::Parse;

pub struct StateParser {
    pub names: syn::ExprArray,
    pub body: syn::Expr,
}

impl Parse for StateParser {
    /// set_state!([array of observables], body)
    /// set_state!(observable, body)
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let names = if let Ok(name) = input.parse::<syn::Ident>() {
            syn::parse(quote! {[#name]}.into())?
        } else {
            input.parse()?
        };

        input.parse::<syn::Token!(,)>()?;
        let body = input.parse::<syn::Expr>()?;

        Ok(Self { names, body })
    }
}
