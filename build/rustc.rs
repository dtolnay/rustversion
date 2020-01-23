use self::Channel::*;

#[derive(Debug)]
pub struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

#[derive(Debug)]
pub enum Channel {
    Stable,
    Beta,
    Nightly(Date),
    Dev,
}

#[derive(Debug)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

pub fn parse(string: &str) -> Option<Version> {
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
        Some(channel) if channel == "nightly" => match words.next() {
            Some(hash) => {
                if !hash.starts_with('(') {
                    return None;
                }
                let date = words.next()?;
                if !date.ends_with(')') {
                    return None;
                }
                let mut date = date[..date.len() - 1].split('-');
                let year = date.next()?.parse().ok()?;
                let month = date.next()?.parse().ok()?;
                let day = date.next()?.parse().ok()?;
                match date.next() {
                    None => Nightly(Date { year, month, day }),
                    Some(_) => return None,
                }
            }
            None => Dev,
        },
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
