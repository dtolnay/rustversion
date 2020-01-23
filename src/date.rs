use crate::time;
use std::fmt::{self, Display};
use syn::parse::{Error, Parse, ParseStream};
use syn::{LitInt, Token};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Display for Date {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day,
        )
    }
}

impl Parse for Date {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let span = input.cursor().token_stream();
        let error = || {
            Error::new_spanned(
                &span,
                format!("expected nightly date, like {}", time::today()),
            )
        };

        let year: LitInt = input.parse().map_err(|_| error())?;
        input.parse::<Token![-]>()?;
        let month: LitInt = input.parse().map_err(|_| error())?;
        input.parse::<Token![-]>()?;
        let day: LitInt = input.parse().map_err(|_| error())?;

        let year = year.base10_parse::<u64>().map_err(|_| error())?;
        let month = month.base10_parse::<u64>().map_err(|_| error())?;
        let day = day.base10_parse::<u64>().map_err(|_| error())?;
        if year >= 3000 || month > 12 || day > 31 {
            return Err(error());
        }

        Ok(Date {
            year: year as u16,
            month: month as u8,
            day: day as u8,
        })
    }
}
