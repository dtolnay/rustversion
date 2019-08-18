use crate::time;
use std::fmt::{self, Display};
use std::num::ParseIntError;
use std::str::FromStr;
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

pub struct ParseDateError;

impl From<ParseIntError> for ParseDateError {
    fn from(_err: ParseIntError) -> Self {
        ParseDateError
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut date = s.split('-');
        let year = date.next().ok_or(ParseDateError)?.parse()?;
        let month = date.next().ok_or(ParseDateError)?.parse()?;
        let day = date.next().ok_or(ParseDateError)?.parse()?;
        match date.next() {
            None => Ok(Date { year, month, day }),
            Some(_) => Err(ParseDateError),
        }
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
