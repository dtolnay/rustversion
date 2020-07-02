use crate::date::Date;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Version {
    pub minor: u16,
    pub patch: u16,
    pub channel: Channel,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)] // Only one of these variants is ever constructed per build
pub enum Channel {
    Stable(Option<Date>),
    Beta(Option<Date>),
    Nightly(Date),
    Dev,
}
