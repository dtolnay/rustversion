use self::Channel::*;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Channel {
    Stable,
    Beta,
    Nightly(Date),
    Dev,
}

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
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
