use crate::date::Date;
use quote::quote;
use std::convert::TryInto;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{LitFloat, LitInt, Token};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Version {
    pub release: Release,
    pub channel: Channel,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Release {
    pub minor: u16,
    pub patch: u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Channel {
    Stable,
    Beta,
    Nightly(Date),
    Dev,
}

impl Parse for Release {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.cursor().token_stream();
        let error = || Error::new_spanned(&span, "expected rustc release number, like 1.31");

        let major_minor: LitFloat = input.parse().map_err(|_| error())?;
        let string = quote!(#major_minor).to_string();

        if !string.starts_with("1.") {
            return Err(error());
        }

        let minor: u16 = string[2..].parse().map_err(|_| error())?;
        let mut patch = 0u16;

        if input.parse::<Option<Token![.]>>()?.is_some() {
            let int: LitInt = input.parse().map_err(|_| error())?;
            patch = int.value().try_into().map_err(|_| error())?;
        }

        Ok(Release { minor, patch })
    }
}
