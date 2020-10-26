use crate::date::Date;
use crate::release::Release;
use crate::version::{Channel::*, Version};
use std::cmp::Ordering;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

pub enum Bound {
    Nightly(Date),
    Stable(Release),
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
