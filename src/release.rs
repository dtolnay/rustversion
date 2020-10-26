use proc_macro2::Literal;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::Token;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Release {
    pub minor: u16,
    pub patch: Option<u16>,
}

impl Parse for Release {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.cursor().token_stream();
        let error = || Error::new_spanned(&span, "expected rustc release number, like 1.31");

        let major_minor: Literal = input.parse().map_err(|_| error())?;
        let string = major_minor.to_string();

        if !string.starts_with("1.") {
            return Err(error());
        }

        let minor: u16 = string[2..].parse().map_err(|_| error())?;

        let patch = if input.parse::<Option<Token![.]>>()?.is_some() {
            let int: Literal = input.parse().map_err(|_| error())?;
            Some(int.to_string().parse().map_err(|_| error())?)
        } else {
            None
        };

        Ok(Release { minor, patch })
    }
}
