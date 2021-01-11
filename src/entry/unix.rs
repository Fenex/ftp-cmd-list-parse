use std::convert::{TryFrom, TryInto};

use ::regex::Regex;

use super::*;

lazy_static! {
    static ref RELIST: Regex = Regex::new(
        r"(?x)
        ^(?P<type>[bcdelfmpSs-])
        (?P<permission>((r|-)(w|-)([xsStTL-]))((r|-)(w|-)([xsStTL-]))((r|-)(w|-)([xsStTL-])))
        (?P<acl>([\+|@]))?\s+
        (?P<inodes>\d+)\s+
        (?P<owner>\d+|[A-Z]{1}\w+\s+[A-Z]{1}\w+|\w+|\S+)\s+
        (?P<group>\d+|[A-Z]{1}[\w\\]+\s+[A-Z]{1}\w+|\w+|\S+)\s+
        (?P<size>\d+(?:,\s*\d*)?)\s+
        (?P<timestamp>((?P<month1>\w{3})\s+
            (?P<date1>\d{1,2})\s+
            (?P<hour>\d{1,2}):(?P<minute>\d{2}))|
            ((?P<month2>\w{3})\s+
                (?P<date2>\d{1,2})\s+
                (?P<year>\d{4})))\s+
        (?P<name>.+)$
    "
    )
    .unwrap();
}

/// Represents entry from Unix-like FTP server.
#[derive(Debug)]
pub struct FtpEntryUnix {
    kind: FtpEntryKind,
    name: String,
    size: usize,
    // pub date: NaiveDateTime,
    date_str: String,
    /// For symlink entries, this is the symlink's target.
    pub target: Option<String>,
    /// True if the sticky bit is set for this entry.
    pub sticky: bool,
    /// The various permissions for this entry.
    pub permissions: FtpEntryPermissions,
    /// Marks extra ACL permission for this entry.
    pub acl: bool,
    /// The user name or ID that this entry belongs to.
    pub owner: String,
    /// The group name or ID that this entry belongs to.
    pub group: String,
    pub pointer: Option<String>,
}

impl FtpEntryUnix {
    /// Represents parsed string as entry of an Unix-type FTP server.
    pub fn new(string: &str) -> Option<Self> {
        FtpEntryUnix::try_from(string).ok()
    }
}

impl FtpEntryInfo for FtpEntryUnix {
    fn kind(&self) -> FtpEntryKind {
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

impl TryFrom<&str> for FtpEntryUnix {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(caps) = RELIST.captures(&value) {
            let kind: FtpEntryKind = caps
                .name("type")
                .ok_or("this is not a list unix format")
                .map_or_else(|_| Err(()), |v| v.as_str().try_into().map_err(|_| ()))?;

            let (sticky, permissions) = caps
                .name("permission")
                .ok_or("this is not a list unix format")
                .map_or_else(
                    |_| Err(()),
                    |v| {
                        let mut permission = v.as_str().to_string();
                        Ok((
                            match permission.chars().last() {
                                Some(t) if t == 't' || t == 'T' => {
                                    permission.pop();
                                    permission.push(if t == 't' { 'x' } else { '-' });
                                    true
                                }
                                _ => false,
                            },
                            permission,
                        ))
                    },
                )?;

            let acl = caps.name("acl").map(|v| v.as_str() == "+").unwrap_or(false);
            let owner = caps.name("owner").unwrap().as_str().to_string();
            let group = caps.name("group").unwrap().as_str().to_string();

            let (size, pointer) = caps.name("size").map_or((0, None), |v| {
                if v.as_str().chars().any(|c| c == ',') {
                    (
                        0,
                        Some(v.as_str().chars().filter(|c| !c.is_whitespace()).collect()),
                    )
                } else {
                    (v.as_str().parse().unwrap_or(0), None)
                }
            });

            let date_str = caps
                .name("timestamp")
                .unwrap()
                .as_str()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            // let date = None
            //     .or_else(|| {
            //         let date = vec![
            //             caps.name("month1"),
            //             caps.name("date1"),
            //             caps.name("hour"),
            //             caps.name("minute"),
            //         ];

            //         let date = if date.iter().any(|m| m.is_none()) {
            //             return None;
            //         } else {
            //             date.iter().map(|m| m.unwrap().as_str()).collect::<Vec<_>>()
            //         };

            //         let now = ::chrono::Utc::now();
            //         let year = format!("{}", now.format("%Y"));

            //         let date = format!("{}-{}-{}T{}:{}", year, date[0], date[1], date[2], date[3]);
            //         let date = match NaiveDateTime::parse_from_str(&date, "%Y-%b-%_dT%_H:%_M") {
            //             Ok(mut date) => {
            //                 // If the date is in the past but no more than 6 months old, year
            //                 // isn't displayed and doesn't have to be the current year.
            //                 //
            //                 // If the date is in the future (less than an hour from now), year
            //                 // isn't displayed and doesn't have to be the current year.
            //                 // That second case is much more rare than the first and less annoying.
            //                 // It's impossible to fix without knowing about the server's timezone,
            //                 // so we just don't do anything about it.
            //                 //
            //                 // If we're here with a time that is more than 28 hours into the
            //                 // future (1 hour + maximum timezone offset which is 27 hours),
            //                 // there is a problem -- we should be in the second conditional block
            //                 if date.timestamp() - now.timestamp() > 100_800_000 {
            //                     date = date.with_year(date.year() - 1).unwrap();
            //                 }

            //                 // If we're here with a time that is more than 6 months old, there's
            //                 // a problem as well.
            //                 // Maybe local & remote servers aren't on the same timezone (with remote
            //                 // ahead of local)
            //                 // For instance, remote is in 2014 while local is still in 2013. In
            //                 // this case, a date like 01/01/13 02:23 could be detected instead of
            //                 // 01/01/14 02:23
            //                 // Our trigger point will be 3600*24*31*6 (since we already use 31
            //                 // as an upper bound, no need to add the 27 hours timezone offset)
            //                 if now.timestamp() - date.timestamp() > 16_070_400_000 {
            //                     date = date.with_year(date.year() - 1).unwrap();
            //                 }

            //                 Some(date)
            //             }
            //             Err(_) => None,
            //         };

            //         date
            //     })
            //     .or_else(|| {
            //         let date = vec![caps.name("year"), caps.name("month2"), caps.name("date2")];

            //         let date = if date.iter().any(|m| m.is_none()) {
            //             return None;
            //         } else {
            //             date.iter().map(|m| m.unwrap().as_str()).collect::<Vec<_>>()
            //         };

            //         let date = format!("{}-{}-{}", date[0], date[1], date[2]);
            //         NaiveDate::parse_from_str(&date, "%Y-%b-%_d")
            //             .map(|date| date.and_hms(0, 0, 0))
            //             .ok()
            //     });

            // let date = match date {
            //     Some(date) => date,
            //     _ => return Err(()),
            // };

            let (name, target) = {
                let name = caps.name("name").unwrap().as_str();
                if kind == FtpEntryKind::Symlink {
                    let mut s1 = name.split(" -> ");
                    (
                        s1.next().unwrap().to_string(),
                        s1.next().map(|v| v.to_string()),
                    )
                } else {
                    (name.to_string(), None)
                }
            };

            return Ok(Self {
                kind,
                name,
                target,
                sticky,
                permissions: FtpEntryPermissions(permissions),
                acl,
                owner,
                group,
                size,
                pointer,
                date_str,
            });
        }

        Err(())
    }
}
