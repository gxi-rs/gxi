use syn::parse::{Parse, ParseStream};

pub trait OptionalParse: Sized + Parse {
    fn optional_parse(input: &ParseStream) -> syn::Result<Option<Self>>;
}

pub fn unwrap_optional_parse<T: OptionalParse>(input: &ParseStream) -> syn::Result<T> {
    if let Some(s) = T::optional_parse(input)? {
        Ok(s)
    } else {
        Err(syn::Error::new(input.span(), "unexpected token"))
    }
}

macro_rules! impl_parse_for_optional_parse {
    ($t:ty) => {
        impl syn::parse::Parse for $t {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                crate::optional_parse::unwrap_optional_parse(&input)
            }
        }
    };
}

pub(crate) use impl_parse_for_optional_parse;
