use crate::bound::{Bound, Release};
use crate::date::Date;
use crate::time;
use crate::version::{Channel, Version};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Token};
use std::sync::RwLock;

pub enum Expr {
    Stable,
    Beta,
    Nightly,
    Date(Date),
    Since(Bound),
    Before(Bound),
    Release(Release),
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
    MinVer(Bound),
}

pub enum ExprError {
}

impl ExprError {
    fn nightly_date(mindate: Date, date: Date) -> Self {
        unimplemented!()
    }

    fn nightly_channel(channel: Channel) -> Self {
        unimplemented!()
    }

    fn nightly_release(release: Release) -> Self {
        unimplemented!()
    }

    fn release(minrel: Release, release: Release) -> Self {
        unimplemented!()
    }

    fn bad_bound(minver: Bound, bound: Bound) -> Self {
        unimplemented!()
    }

    fn duplicate_minver(minver: Bound, new_minver: Bound) -> Self {
        unimplemented!()
    }
}

pub type ExprResult<T> = std::result::Result<T, ExprError>;

lazy_static! {
    static ref MINVER: RwLock<Option<Bound>> = RwLock::new(None);
}

fn check_minver_channel(minver: Option<Bound>, channel: Channel) -> ExprResult<()> {
    match minver {
        Some(Bound::Nightly(mindate)) => {
            match channel {
                Channel::Nightly(checkdate) => if checkdate < mindate {
                    Err(ExprError::nightly_date(mindate, checkdate))
                } else {
                    Ok(())
                },
                Channel::Stable | Channel::Beta => Err(ExprError::nightly_channel(channel)),
                _ => Ok(()),
            }
        },
        _ => Ok(()),
    }
}

fn check_minver_bound(minver: Option<Bound>, bound: &Bound) -> ExprResult<()> {
    match minver {
        Some(minver) => if bound < &minver {
            Err(ExprError::bad_bound(minver, *bound))
        } else {
            Ok(())
        },
        None => Ok(()),
    }
}

fn check_minver_release(minver: Option<Bound>, release: &Release) -> ExprResult<()> {
    match minver {
        Some(Bound::Nightly(_)) => Err(ExprError::nightly_release(*release)),
        Some(Bound::Stable(minrel)) => if release < &minrel {
            Err(ExprError::release(minrel, *release))
        } else {
            Ok(())
        },
        None => Ok(()),
    }
}

impl Expr {
    pub fn eval(&self, rustc: Version) -> ExprResult<bool> {
        use self::Expr::*;

        let minver = {
            let bound = MINVER.read().expect("minver lock poisoned");
            *bound
        };

        Ok(match self {
            Stable => {
                check_minver_channel(minver, Channel::Stable)?;
                rustc.channel == Channel::Stable
            },
            Beta => {
                check_minver_channel(minver, Channel::Beta)?;
                rustc.channel == Channel::Beta
            },
            Nightly => {
                check_minver_channel(minver, Channel::Nightly(time::today()))?;
                match rustc.channel {
                    Channel::Nightly(_) | Channel::Dev => true,
                    Channel::Stable | Channel::Beta => false,
                }
            },
            Date(date) => {
                check_minver_channel(minver, Channel::Nightly(*date))?;
                match rustc.channel {
                    Channel::Nightly(rustc) => rustc == *date,
                    Channel::Stable | Channel::Beta | Channel::Dev => false,
                }
            },
            Since(bound) => {
                check_minver_bound(minver, bound)?;
                rustc >= *bound
            },
            Before(bound) => {
                check_minver_bound(minver, bound)?;
                rustc < *bound
            },
            Release(release) => {
                check_minver_release(minver, release)?;
                rustc.channel == Channel::Stable
                    && rustc.minor == release.minor
                    && release.patch.map_or(true, |patch| rustc.patch == patch)
            }
            Not(expr) => !expr.eval(rustc)?,
            Any(exprs) => exprs.iter().map(|e| e.eval(rustc)).collect::<ExprResult<Vec<_>>>()?.into_iter().any(|b| b),
            All(exprs) => exprs.iter().map(|e| e.eval(rustc)).collect::<ExprResult<Vec<_>>>()?.into_iter().all(|b| b),
            MinVer(bound) => {
                if let Some(minver) = minver {
                    Err(ExprError::duplicate_minver(minver, *bound))?
                } else {
                    let mut new_minver = MINVER.write().expect("minver lock poisoned");
                    *new_minver = Some(*bound);
                    true
                }
            },
        })
    }
}

type Exprs = Punctuated<Expr, Token![,]>;

mod keyword {
    syn::custom_keyword!(stable);
    syn::custom_keyword!(beta);
    syn::custom_keyword!(nightly);
    syn::custom_keyword!(since);
    syn::custom_keyword!(before);
    syn::custom_keyword!(not);
    syn::custom_keyword!(any);
    syn::custom_keyword!(all);
    syn::custom_keyword!(minver);
}

impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(keyword::stable) {
            Self::parse_stable(input)
        } else if lookahead.peek(keyword::beta) {
            Self::parse_beta(input)
        } else if lookahead.peek(keyword::nightly) {
            Self::parse_nightly(input)
        } else if lookahead.peek(keyword::since) {
            Self::parse_since(input)
        } else if lookahead.peek(keyword::before) {
            Self::parse_before(input)
        } else if lookahead.peek(keyword::not) {
            Self::parse_not(input)
        } else if lookahead.peek(keyword::any) {
            Self::parse_any(input)
        } else if lookahead.peek(keyword::all) {
            Self::parse_all(input)
        } else if lookahead.peek(keyword::minver) {
            Self::parse_minver(input)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Expr {
    fn parse_nightly(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::nightly>()?;

        if !input.peek(token::Paren) {
            return Ok(Expr::Nightly);
        }

        let paren;
        parenthesized!(paren in input);
        let date: Date = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::Date(date))
    }

    fn parse_beta(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::beta>()?;

        Ok(Expr::Beta)
    }

    fn parse_stable(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::stable>()?;

        if !input.peek(token::Paren) {
            return Ok(Expr::Stable);
        }

        let paren;
        parenthesized!(paren in input);
        let release: Release = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::Release(release))
    }

    fn parse_since(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::since>()?;

        let paren;
        parenthesized!(paren in input);
        let bound: Bound = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::Since(bound))
    }

    fn parse_before(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::before>()?;

        let paren;
        parenthesized!(paren in input);
        let bound: Bound = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::Before(bound))
    }

    fn parse_not(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::not>()?;

        let paren;
        parenthesized!(paren in input);
        let expr: Expr = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::Not(Box::new(expr)))
    }

    fn parse_any(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::any>()?;

        let paren;
        parenthesized!(paren in input);
        let exprs: Exprs = paren.parse_terminated(Expr::parse)?;

        Ok(Expr::Any(exprs.into_iter().collect()))
    }

    fn parse_all(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::all>()?;

        let paren;
        parenthesized!(paren in input);
        let exprs: Exprs = paren.parse_terminated(Expr::parse)?;

        Ok(Expr::All(exprs.into_iter().collect()))
    }

    fn parse_minver(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::minver>()?;

        let paren;
        parenthesized!(paren in input);
        let bound: Bound = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::MinVer(bound))
    }
}
