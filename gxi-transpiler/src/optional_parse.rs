use syn::parse::{Parse, ParseStream};

pub trait OptionalParse: Sized + Parse {
    fn optional_parse(input: ParseStream) -> syn::Result<Option<Self>>;
}

impl<T: OptionalParse> Parse for T {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        unwrap_optional_parse(&input)
    }
}

pub fn unwrap_optional_parse<T: OptionalParse>(input: ParseStream) -> syn::Result<T> {
    if let Some(s) = T::optional_parse(input)? {
        Ok(s)
    } else {
        Err(syn::Error::new(input.span(), "unexpected token"))
    }
}
