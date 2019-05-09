use crate::date::Date;
use crate::version::{Channel::*, Version};
use quote::quote;
use std::cmp::Ordering;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{LitFloat, LitInt, Token};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord)]
pub enum Bound {
    Nightly(Date),
    Stable(Release),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Release {
    pub minor: u16,
    pub patch: Option<u16>,
}

impl Parse for Bound {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek2(Token![-]) {
            input.parse().map(Bound::Nightly)
        } else {
            input.parse().map(Bound::Stable)
        }
    }
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

        let patch = if input.parse::<Option<Token![.]>>()?.is_some() {
            let int: LitInt = input.parse().map_err(|_| error())?;
            Some(int.value().to_string().parse().map_err(|_| error())?)
        } else {
            None
        };

        Ok(Release { minor, patch })
    }
}

impl PartialEq<Bound> for Version {
    fn eq(&self, rhs: &Bound) -> bool {
        match rhs {
            Bound::Nightly(date) => match self.channel {
                Stable | Beta | Dev => false,
                Nightly(nightly) => nightly == *date,
            },
            Bound::Stable(release) => {
                self.minor == release.minor
                    && release.patch.map_or(true, |patch| self.patch == patch)
            }
        }
    }
}

impl PartialOrd<Bound> for Version {
    fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering> {
        match rhs {
            Bound::Nightly(date) => match self.channel {
                Stable | Beta => Some(Ordering::Less),
                Nightly(nightly) => Some(nightly.cmp(date)),
                Dev => Some(Ordering::Greater),
            },
            Bound::Stable(release) => {
                let version = (self.minor, self.patch);
                let bound = (release.minor, release.patch.unwrap_or(0));
                Some(version.cmp(&bound))
            }
        }
    }
}

impl PartialOrd<Bound> for Bound {
    fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering> {
        Some(match (self, rhs) {
            (Bound::Nightly(ref date1), Bound::Nightly(ref date2)) => date1.cmp(date2),
            (Bound::Nightly(_), Bound::Stable(_)) => Ordering::Greater,
            (Bound::Stable(_), Bound::Nightly(_)) => Ordering::Less,
            (Bound::Stable(ref release1), Bound::Stable(ref release2)) => release1.cmp(release2),
        })
    }
}
