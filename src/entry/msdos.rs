use std::convert::TryFrom;

use ::regex::Regex;

use super::*;

lazy_static! {
    static ref RELIST: Regex = Regex::new(
        r"(?x)
        ^(?P<month>\d{2})(?:\-|/)
        (?P<date>\d{2})(?:\-|/)
        (?P<year>\d{2,4})\s+
        (?P<hour>\d{2}):(?P<minute>\d{2})\s{0,1}(?P<ampm>[AaMmPp]{1,2})\s+
        (?:(?P<size>\d+)|(?P<isdir><DIR>))\s+
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

impl FtpEntryMsdos {
    /// Represents parsed string as entry of a MSDOS-like FTP server.
    pub fn new(string: &str) -> Option<Self> {
        FtpEntryMsdos::try_from(string).ok()
    }
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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(caps) = RELIST.captures(&value) {
            let as_str = |s| caps.name(s).unwrap().as_str();

            let name = as_str("name");
            let kind = caps
                .name("isdir")
                .map_or(FtpEntryKind::File, |_| FtpEntryKind::Directory);
            let size = caps
                .name("size")
                .map(|s| s.as_str().parse::<usize>())
                .map_or(0, |r| r.map_or(0, |size| size));

            let date_str = {
                let month: u8 = as_str("month").parse().unwrap();
                let date: u8 = as_str("date").parse().unwrap();
                let year: u32 = match (as_str("year").len(), as_str("year").parse::<u32>().unwrap())
                {
                    (len, year) if len < 4 => year + if year < 70 { 2000 } else { 1900 },
                    (_, year) => year,
                };
                let mut hour: u8 = as_str("hour").parse().unwrap();
                let minute: u8 = as_str("minute").parse().unwrap();
                if hour < 12
                    && as_str("ampm")
                        .bytes()
                        .next()
                        .unwrap()
                        .eq_ignore_ascii_case(&b'p')
                {
                    hour += 12;
                } else if hour == 12
                    && as_str("ampm")
                        .bytes()
                        .next()
                        .unwrap()
                        .eq_ignore_ascii_case(&b'a')
                {
                    hour = 0;
                }

                format!(
                    "{}-{:02}-{:02}T{:02}:{:02}",
                    year, month, date, hour, minute
                )
            };

            return Ok(Self {
                name: name.to_owned(),
                kind,
                size,
                date_str,
            });
        }

        Err(())
    }
}
