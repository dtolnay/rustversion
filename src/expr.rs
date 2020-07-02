use crate::bound::{Bound, Release};
use crate::date::Date;
use crate::version::{Channel, Version};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parenthesized, token, Token};

pub enum Expr {
    Stable,
    Beta,
    Nightly,
    Date(Date),
    Since(Bound),
    SinceDate(Date),
    Before(Bound),
    BeforeDate(Date),
    Release(Release),
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
}

// Compat for older versions of Rust
macro_rules! matches {
    ($expression:expr, $( $pattern:pat )|+) => {
        match $expression {
            $( $pattern )|+ => true,
            _ => false
        }
    }
}


impl Expr {
    pub fn eval(&self, rustc: Version) -> bool {
        use self::Expr::*;

        let since_date = |bound_date| {
            match rustc.channel {
                Channel::Stable(date) | Channel::Beta(date) => {
                    date.map_or(false, |d| d >= bound_date)
                }
                Channel::Nightly(date) => {
                    date >= bound_date
                }
                Channel::Dev => true
            }
        };

        match self {
            Stable => matches!(rustc.channel, Channel::Stable(_)),
            Beta => matches!(rustc.channel, Channel::Beta(_)),
            Nightly => match rustc.channel {
                Channel::Nightly(_) | Channel::Dev => true,
                Channel::Stable(_) | Channel::Beta(_) => false,
            },
            Date(date) => match rustc.channel {
                Channel::Nightly(rustc) => rustc == *date,
                Channel::Stable(_) | Channel::Beta(_) | Channel::Dev => false,
            },
            Since(bound) => rustc >= *bound,
            SinceDate(bound) => since_date(*bound),
            Before(bound) => rustc < *bound,
            BeforeDate(bound) => !since_date(*bound),
            Release(release) => {
                matches!(rustc.channel, Channel::Stable(_))
                    && rustc.minor == release.minor
                    && release.patch.map_or(true, |patch| rustc.patch == patch)
            }
            Not(expr) => !expr.eval(rustc),
            Any(exprs) => exprs.iter().any(|e| e.eval(rustc)),
            All(exprs) => exprs.iter().all(|e| e.eval(rustc)),
        }
    }
}

type Exprs = Punctuated<Expr, Token![,]>;

mod keyword {
    syn::custom_keyword!(stable);
    syn::custom_keyword!(beta);
    syn::custom_keyword!(nightly);
    syn::custom_keyword!(since);
    syn::custom_keyword!(since_date);
    syn::custom_keyword!(before);
    syn::custom_keyword!(before_date);
    syn::custom_keyword!(not);
    syn::custom_keyword!(any);
    syn::custom_keyword!(all);
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
        } else if lookahead.peek(keyword::since_date) {
            Self::parse_since_date(input)
        } else if lookahead.peek(keyword::before) {
            Self::parse_before(input)
        } else if lookahead.peek(keyword::before_date) {
            Self::parse_before_date(input)
        } else if lookahead.peek(keyword::not) {
            Self::parse_not(input)
        } else if lookahead.peek(keyword::any) {
            Self::parse_any(input)
        } else if lookahead.peek(keyword::all) {
            Self::parse_all(input)
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

    fn parse_since_date(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::since_date>()?;

        let paren;
        parenthesized!(paren in input);
        let date: Date = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::SinceDate(date))

    }

    fn parse_before_date(input: ParseStream) -> Result<Self> {
        input.parse::<keyword::before_date>()?;

        let paren;
        parenthesized!(paren in input);
        let date: Date = paren.parse()?;
        paren.parse::<Option<Token![,]>>()?;

        Ok(Expr::BeforeDate(date))

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
}
