use syn::parse::Parse;

pub struct StateParser {
    pub name: syn::Ident,
    pub body: syn::Expr,
}

impl Parse for StateParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token!(,)>()?;
        let body = input.parse::<syn::Expr>()?;

        Ok(Self {
            name,
            body
        })
    }
}
