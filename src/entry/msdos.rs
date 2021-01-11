use std::convert::TryFrom;

use ::regex::Regex;

use super::*;

lazy_static! {
    static ref RELIST: Regex = Regex::new(
        r"(?x)
        ^(?P<month>\d{2})(?:\-|\/)
        (?P<date>\d{2})(?:\-|\/)
        (?P<year>\d{2,4})\s+
        (?P<hour>\d{2}):(?P<minute>\d{2})\s{0,1}(?P<ampm>[AaMmPp]{1,2})\s+
        (?:(?P<size>\d+)|(?P<isdir>\<DIR\>))\s+
        (?P<name>.+)$
        "
    )
    .unwrap();
}

/// Represents entry from Msdos-like FTP server.
#[derive(Debug)]
pub struct FtpEntryMsdos {
    kind: FtpEntryKind,
    name: String,
    size: usize,
    // date: NaiveDateTime,
    date_str: String,
}

impl FtpEntryInfo for FtpEntryMsdos {
    fn kind(&self) -> super::FtpEntryKind {
        self.kind
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn size(&self) -> usize {
        self.size
    }

    // fn date(&self) -> NaiveDateTime {
    //     self.date
    // }

    fn date_str(&self) -> &str {
        &self.date_str
    }
}

impl TryFrom<&str> for FtpEntryMsdos {
    type Error = ();

    fn try_from(_value: &str) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}
