use std::env;
use std::ffi::OsString;
use std::fmt::{self, Display};
use std::io;
use std::process::Command;
use std::str::FromStr;
use std::string::FromUtf8Error;

use crate::date::Date;
use crate::version::{Channel::*, Version};
use proc_macro2::Span;

#[derive(Debug)]
pub enum Error {
    Exec(io::Error),
    Utf8(FromUtf8Error),
    Parse(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            Exec(e) => write!(f, "failed to run `rustc --version`: {}", e),
            Utf8(e) => write!(f, "failed to parse output of `rustc --version`: {}", e),
            Parse(string) => write!(
                f,
                "unexpected output from `rustc --version`, please file an issue: {:?}",
                string,
            ),
        }
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::Utf8(err)
    }
}

impl From<Error> for syn::Error {
    fn from(err: Error) -> Self {
        syn::Error::new(Span::call_site(), err)
    }
}

pub fn version() -> Result<Version> {
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| OsString::from("rustc"));
    let output = Command::new(rustc)
        .arg("--version")
        .output()
        .map_err(Error::Exec)?;
    let string = String::from_utf8(output.stdout)?;

    match parse(&string) {
        Some(version) => Ok(version),
        None => Err(Error::Parse(string)),
    }
}

fn parse(string: &str) -> Option<Version> {
    let last_line = string.lines().last().unwrap_or(&string);
    let mut words = last_line.trim().split(' ');

    if words.next()? != "rustc" {
        return None;
    }

    let mut version_channel = words.next()?.split('-');
    let version = version_channel.next()?;
    let channel = version_channel.next();

    let mut digits = version.split('.');
    let major = digits.next()?;
    if major != "1" {
        return None;
    }
    let minor = digits.next()?.parse().ok()?;
    let patch = digits.next().unwrap_or("0").parse().ok()?;

    let channel = match channel {
        None => Stable,
        Some(channel) if channel == "dev" => Dev,
        Some(channel) if channel.starts_with("beta") => Beta,
        Some(channel) if channel == "nightly" => {
            match words.next() {
                Some(hash) => {
                    if !hash.starts_with('(') {
                        return None;
                    }
                    let date = words.next()?;
                    if !date.ends_with(')') {
                        return None;
                    }
                    let date = Date::from_str(&date[..date.len() - 1]).ok()?;
                    Nightly(date)
                }
                None => Dev,
            }
        }
        Some(_) => return None,
    };

    Some(Version {
        minor,
        patch,
        channel,
    })
}

#[test]
fn test_parse() {
    let cases = &[
        (
            "rustc 1.0.0 (a59de37e9 2015-05-13) (built 2015-05-14)",
            Version {
                minor: 0,
                patch: 0,
                channel: Stable,
            },
        ),
        (
            "rustc 1.18.0",
            Version {
                minor: 18,
                patch: 0,
                channel: Stable,
            },
        ),
        (
            "rustc 1.24.1 (d3ae9a9e0 2018-02-27)",
            Version {
                minor: 24,
                patch: 1,
                channel: Stable,
            },
        ),
        (
            "rustc 1.35.0-beta.3 (c13114dc8 2019-04-27)",
            Version {
                minor: 35,
                patch: 0,
                channel: Beta,
            },
        ),
        (
            "rustc 1.36.0-nightly (938d4ffe1 2019-04-27)",
            Version {
                minor: 36,
                patch: 0,
                channel: Nightly(Date {
                    year: 2019,
                    month: 4,
                    day: 27,
                }),
            },
        ),
        (
            "rustc 1.36.0-dev",
            Version {
                minor: 36,
                patch: 0,
                channel: Dev,
            },
        ),
        (
            "rustc 1.36.0-nightly",
            Version {
                minor: 36,
                patch: 0,
                channel: Dev,
            },
        ),
        (
            "warning: invalid logging spec 'warning', ignoring it
             rustc 1.30.0-nightly (3bc2ca7e4 2018-09-20)",
            Version {
                minor: 30,
                patch: 0,
                channel: Nightly(Date {
                    year: 2018,
                    month: 9,
                    day: 20,
                }),
            },
        ),
    ];

    for (string, expected) in cases {
        assert_eq!(parse(string).unwrap(), *expected);
    }
}
