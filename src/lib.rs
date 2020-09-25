#[macro_use]
extern crate lazy_static;

mod expressions;

use std::convert::TryFrom;
use std::convert::TryInto;

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use regex::Match;

use expressions::*;

#[derive(Debug, PartialEq)]
pub enum FtpFileType {
    UNKNOWN,
    Directory,
    File,
    BlockDevice,
    CharacterDevice,
    Pipe,
    Socket,
    Symlink,
}

impl From<char> for FtpFileType {
    fn from(value: char) -> Self {
        match value {
            '-' => Self::File,
            'd' => Self::Directory,
            'b' => Self::BlockDevice,
            'c' => Self::CharacterDevice,
            'p' => Self::Pipe,
            's' => Self::Socket,
            'l' => Self::Symlink,
            _ => Self::UNKNOWN,
        }
    }
}

impl TryFrom<&str> for FtpFileType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            Ok(value.chars().nth(0).unwrap().into())
        } else {
            Err("length of the value must be equal to 1")
        }
    }
}

#[derive(Debug)]
pub struct FtpFile {
    entity: FtpFileType,
    name: String,
    target: Option<String>,
    sticky: bool,
    permissions: String,
    acl: bool,
    owner: String,
    group: String,
    size: usize,
    pointer: Option<String>,
    date: Option<NaiveDateTime>,
}

impl TryFrom<&str> for FtpFile {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(caps) = REX_LISTUNIX.captures(&value) {
            let entity: FtpFileType = caps
                .name("type")
                .ok_or("this is not a list unix format")
                .map_or_else(|e| Err(e), |v| v.as_str().try_into())?;

            let (sticky, permissions) = caps
                .name("permission")
                .ok_or("this is not a list unix format")
                .map_or_else(
                    |e| Err(e),
                    |v: Match| {
                        let mut permission = v.as_str().to_string();
                        Ok((
                            match permission.chars().last() {
                                Some(t) if t == 't' || t == 'T' => {
                                    permission.pop();
                                    if t == 't' {
                                        permission.push('x');
                                    } else {
                                        permission.push('-');
                                    }
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

            let date = None
                .or_else(|| {
                    let date = vec![
                        caps.name("month1"),
                        caps.name("date1"),
                        caps.name("hour"),
                        caps.name("minute"),
                    ];

                    let date = if date.iter().any(|m| m.is_none()) {
                        return None;
                    } else {
                        date.iter().map(|m| m.unwrap().as_str()).collect::<Vec<_>>()
                    };

                    let now = ::chrono::Utc::now();
                    let year = format!("{}", now.format("%Y"));

                    let date = format!("{}-{}-{}T{}:{}", year, date[0], date[1], date[2], date[3]);
                    let date = match NaiveDateTime::parse_from_str(&date, "%Y-%b-%_dT%_H:%_M") {
                        Ok(mut date) => {
                            // If the date is in the past but no more than 6 months old, year
                            // isn't displayed and doesn't have to be the current year.
                            //
                            // If the date is in the future (less than an hour from now), year
                            // isn't displayed and doesn't have to be the current year.
                            // That second case is much more rare than the first and less annoying.
                            // It's impossible to fix without knowing about the server's timezone,
                            // so we just don't do anything about it.
                            //
                            // If we're here with a time that is more than 28 hours into the
                            // future (1 hour + maximum timezone offset which is 27 hours),
                            // there is a problem -- we should be in the second conditional block
                            if date.timestamp() - now.timestamp() > 100_800_000 {
                                date = date.with_year(date.year() - 1).unwrap();
                            }

                            // If we're here with a time that is more than 6 months old, there's
                            // a problem as well.
                            // Maybe local & remote servers aren't on the same timezone (with remote
                            // ahead of local)
                            // For instance, remote is in 2014 while local is still in 2013. In
                            // this case, a date like 01/01/13 02:23 could be detected instead of
                            // 01/01/14 02:23
                            // Our trigger point will be 3600*24*31*6 (since we already use 31
                            // as an upper bound, no need to add the 27 hours timezone offset)
                            if now.timestamp() - date.timestamp() > 16_070_400_000 {
                                date = date.with_year(date.year() - 1).unwrap();
                            }

                            Some(date)
                        }
                        Err(_) => None,
                    };

                    date
                })
                .or_else(|| {
                    let date = vec![caps.name("year"), caps.name("month2"), caps.name("date2")];

                    let date = if date.iter().any(|m| m.is_none()) {
                        return None;
                    } else {
                        date.iter().map(|m| m.unwrap().as_str()).collect::<Vec<_>>()
                    };

                    let date = format!("{}-{}-{}", date[0], date[1], date[2]);
                    NaiveDate::parse_from_str(&date, "%Y-%b-%_d")
                        .map(|date| date.and_hms(0, 0, 0))
                        .ok()
                });

            let (name, target) = {
                let name = caps.name("name").unwrap().as_str();
                if entity == FtpFileType::Symlink {
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
                entity,
                name,
                target,
                sticky,
                permissions,
                acl,
                owner,
                group,
                size,
                pointer,
                date,
            });
        }

        if let Some(_caps) = REX_LISTMSDOS.captures(&value) {
            // TODO
        }

        Err("parse fail")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod linux {
        use super::*;

        #[test]
        fn normal_directory() {
            let row = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";

            let ftpfile = FtpFile::try_from(row);
            assert!(ftpfile.is_ok());
            let ftpfile = ftpfile.unwrap();

            assert_eq!(ftpfile.entity, FtpFileType::Directory);
            assert_eq!(ftpfile.name, "usr");
            assert_eq!(ftpfile.target, None);
            assert_eq!(ftpfile.sticky, false);
            assert_eq!(ftpfile.permissions, "rwxr-xr-x");
            assert_eq!(ftpfile.acl, false);
            assert_eq!(ftpfile.owner, "root");
            assert_eq!(ftpfile.group, "root");
            assert_eq!(ftpfile.size, 4096);
            assert_eq!(ftpfile.pointer, None);
            assert_eq!(
                ftpfile.date.unwrap(),
                NaiveDate::from_ymd(2012, 12, 21).and_hms(0, 0, 0)
            );
        }
    }

    #[test]
    fn normal_directory_2() {
        let row = "drwxrwxrwx   1 owner   group          0 Aug 31 2012 e-books";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "e-books");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxrwx");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, None);
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 8, 31).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn normal_file() {
        let row = "-rw-rw-rw-   1 owner   group    7045120 Sep 02  2012 music.mp3";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "music.mp3");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw-rw-");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 7045120);
        assert_eq!(ftpfile.pointer, None);
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn file_with_number_id_owner() {
        let row = "-rw-rw-rw-   1 1234   group    7045120 Sep 02  2012 music.mp3";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "music.mp3");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw-rw-");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "1234");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 7045120);
        assert_eq!(ftpfile.pointer, None);
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn file_with_number_id_group() {
        let row = "-rw-rw-rw-   1 owner   1234    7045120 Sep 02  2012 music.mp3";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "music.mp3");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw-rw-");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "1234");
        assert_eq!(ftpfile.size, 7045120);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn file_with_space_in_group() {
        let row =
            "-rwxrwxr-x    1 1317       Domain Use                3065 May  4 11:01 xmlrpc.php";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "xmlrpc.php");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "1317");
        assert_eq!(ftpfile.group, "Domain Use");
        assert_eq!(ftpfile.size, 3065);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 4).and_hms(11, 1, 0)
        );
    }

    #[test]
    fn file_with_double_space_in_group() {
        let row =
            "-rwxrwxr-x    1 1317       Domain  Use                3065 May  4 11:01 xmlrpc.php";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "xmlrpc.php");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "1317");
        assert_eq!(ftpfile.group, "Domain  Use");
        assert_eq!(ftpfile.size, 3065);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 4).and_hms(11, 1, 0)
        );
    }

    #[test]
    fn file_with_space_in_owner_name() {
        let row = "-rwxrwxr-x    1 Domain Use       33                3065 May  4 11:01 xmlrpc.php";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "xmlrpc.php");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "Domain Use");
        assert_eq!(ftpfile.group, "33");
        assert_eq!(ftpfile.size, 3065);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 4).and_hms(11, 1, 0)
        );
    }

    #[test]
    fn file_with_double_space_in_owner_name() {
        let row =
            "-rwxrwxr-x    1 Domain  Use       33                3065 May  4 11:01 xmlrpc.php";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "xmlrpc.php");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "Domain  Use");
        assert_eq!(ftpfile.group, "33");
        assert_eq!(ftpfile.size, 3065);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 4).and_hms(11, 1, 0)
        );
    }

    #[test]
    fn file_with_number_owner_and_hyphen_groupname() {
        let row = "-rw-------    1 33         www-data           14 May 15 01:52 .ftpquota";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, ".ftpquota");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-------");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "33");
        assert_eq!(ftpfile.group, "www-data");
        assert_eq!(ftpfile.size, 14);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 15).and_hms(1, 52, 0)
        );
    }

    #[test]
    fn file_with_hyphen_owner_and_number_groupname() {
        let row = "-rw-------    1 www-data         33           14 May 15 01:52 .ftpquota";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, ".ftpquota");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-------");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "www-data");
        assert_eq!(ftpfile.group, "33");
        assert_eq!(ftpfile.size, 14);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 15).and_hms(1, 52, 0)
        );
    }

    #[test]
    fn file_with_acl_set() {
        let row = "-rw-rw-rw-+   1 owner   group    7045120 Sep 02  2012 music.mp3";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "music.mp3");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw-rw-");
        assert_eq!(ftpfile.acl, true);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 7045120);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_sticky_bit_and_executable_for_others() {
        let row = "drwxrwxrwt   7 root   root    4096 May 19 2012 tmp";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "tmp");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, true);
        assert_eq!(ftpfile.permissions, "rwxrwxrwx");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "root");
        assert_eq!(ftpfile.size, 4096);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_sticky_bit_and_executable_for_others_2() {
        let row = "drwxrwx--t   7 root   root    4096 May 19 2012 tmp";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "tmp");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, true);
        assert_eq!(ftpfile.permissions, "rwxrwx--x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "root");
        assert_eq!(ftpfile.size, 4096);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_sticky_bit_and_not_executable_for_others() {
        let row = "drwxrwxrwT   7 root   root    4096 May 19 2012 tmp";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "tmp");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, true);
        assert_eq!(ftpfile.permissions, "rwxrwxrw-");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "root");
        assert_eq!(ftpfile.size, 4096);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_sticky_bit_and_not_executable_for_others_2() {
        let row = "drwxrwx--T   7 root   root    4096 May 19 2012 tmp";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "tmp");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, true);
        assert_eq!(ftpfile.permissions, "rwxrwx---");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "root");
        assert_eq!(ftpfile.size, 4096);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_space_in_group_name() {
        let row = "drwxrwxr-x    7 1317       Domain Use        208 May  5 11:28 wp-content";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "wp-content");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "1317");
        assert_eq!(ftpfile.group, "Domain Use");
        assert_eq!(ftpfile.size, 208);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 5).and_hms(11, 28, 0)
        );
    }

    #[test]
    fn directory_with_double_space_in_group_name() {
        let row = "drwxrwxr-x    7 1317       Domain  Use        208 May  5 11:28 wp-content";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "wp-content");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "1317");
        assert_eq!(ftpfile.group, "Domain  Use");
        assert_eq!(ftpfile.size, 208);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 5).and_hms(11, 28, 0)
        );
    }

    #[test]
    fn directory_with_space_in_group_name_and_owner_name() {
        let row = "drwxrwxr-x    7 Domain Use       Domain Use        208 May  5 11:28 wp-content";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "wp-content");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "Domain Use");
        assert_eq!(ftpfile.group, "Domain Use");
        assert_eq!(ftpfile.size, 208);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 5, 5).and_hms(11, 28, 0)
        );
    }

    #[test]
    fn directory_with_undeifned_bit_state() {
        let row = "drwxr-S---    3 105207   501            18 Jul 04  2017 .pki";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, ".pki");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxr-S---");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "105207");
        assert_eq!(ftpfile.group, "501");
        assert_eq!(ftpfile.size, 18);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn directory_with_set_the_setUserID_or_setGroupID_bit() {
        let row = "drwxr-s---    3 105207   501            18 Jul 04  2017 .pki";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, ".pki");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxr-s---");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "105207");
        assert_eq!(ftpfile.group, "501");
        assert_eq!(ftpfile.size, 18);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_mandatory_lock() {
        let row = "drwx--L---    3 105207   501            18 Jul 04  2017 .pki";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, ".pki");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwx--L---");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "105207");
        assert_eq!(ftpfile.group, "501");
        assert_eq!(ftpfile.size, 18);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_block_device_simple() {
        let row = "brwx-w----    3 105207   501            18 Jul 04  2017 .pki";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::BlockDevice);
        assert_eq!(ftpfile.name, ".pki");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwx-w----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "105207");
        assert_eq!(ftpfile.group, "501");
        assert_eq!(ftpfile.size, 18);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_block_device() {
        let row = "brw-rw----  1 root disk    8,   0 Nov 24 10:13 sda";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::BlockDevice);
        assert_eq!(ftpfile.name, "sda");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "disk");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, Some("8,0".to_string()));

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 11, 24).and_hms(10, 13, 0)
        );
    }

    #[test]
    fn directory_with_character_device_without_pointer() {
        let row = "crw-rw----  1 root tty       0 Apr  1 20:30 vcs";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::CharacterDevice);
        assert_eq!(ftpfile.name, "vcs");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "tty");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 4, 1).and_hms(20, 30, 0)
        );
    }

    #[test]
    fn directory_with_character_device_with_pointer() {
        let row = "crw-rw---- 1 root tty       7, 134 Apr  1 20:30 vcsa6";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::CharacterDevice);
        assert_eq!(ftpfile.name, "vcsa6");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "tty");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, Some("7,134".to_string()));

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 4, 1).and_hms(20, 30, 0)
        );
    }

    #[test]
    fn directory_with_character_device() {
        let row = "crw-rw----  1 root tty       7,   0 Apr  1 20:30 vcs";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::CharacterDevice);
        assert_eq!(ftpfile.name, "vcs");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "tty");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, Some("7,0".to_string()));

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 4, 1).and_hms(20, 30, 0)
        );
    }

    #[test]
    fn directory_with_named_pipe() {
        let row = "prwx-w----    3 105207   501            18 Jul 04  2017 .pki";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Pipe);
        assert_eq!(ftpfile.name, ".pki");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwx-w----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "105207");
        assert_eq!(ftpfile.group, "501");
        assert_eq!(ftpfile.size, 18);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_socket() {
        let row = "srwx-w----    3 105207   501            18 Jul 04  2017 .pki";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Socket);
        assert_eq!(ftpfile.name, ".pki");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwx-w----");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "105207");
        assert_eq!(ftpfile.group, "501");
        assert_eq!(ftpfile.size, 18);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_symlink() {
        let row = "lrwxrwxrwx 1 root root 51 Apr  4 23:57 www.nodeftp.github -> /etc/nginx/sites-available/www.nodeftp.github";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Symlink);
        assert_eq!(ftpfile.name, "www.nodeftp.github");
        assert_eq!(
            ftpfile.target,
            Some("/etc/nginx/sites-available/www.nodeftp.github".to_string())
        );
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxrwx");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "root");
        assert_eq!(ftpfile.size, 51);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 4, 4).and_hms(23, 57, 0)
        );
    }

    #[test]
    fn macos_special_symbol_file() {
        let row = "-rw-rw-rw-@   1 owner   group    7045120 Sep 02  2012 music.mp3";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "music.mp3");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-rw-rw-");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 7045120);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn directory_with_special_name_2() {
        let row = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 1.1 Header [13]";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "1.1 Header [13]");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxr-xr-x");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "root");
        assert_eq!(ftpfile.group, "root");
        assert_eq!(ftpfile.size, 4096);
        assert_eq!(ftpfile.pointer, None);

        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(2012, 12, 21).and_hms(0, 0, 0)
        );
    }

    #[test]
    fn windows_with_unix_style_directory_1() {
        let row = "drwxrwxrwx   1 owner    group               0 Aug 22 14:05 wwwroot";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "wwwroot");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxrwx");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 8, 22).and_hms(14, 5, 0)
        );
    }

    #[test]
    fn windows_with_unix_style_directory_2() {
        let row = "drwxrwxrwx   1 owner    group               0 Aug 22 14:05 Name []";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::Directory);
        assert_eq!(ftpfile.name, "Name []");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxrwx");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 0);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 8, 22).and_hms(14, 5, 0)
        );
    }

    #[test]
    fn windows_with_unix_style_file() {
        let row = "-rwxrwxrwx   1 owner    group           99710 Aug 22 12:59 iisstart.png";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, "iisstart.png");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rwxrwxrwx");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "owner");
        assert_eq!(ftpfile.group, "group");
        assert_eq!(ftpfile.size, 99710);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 8, 22).and_hms(12, 59, 0)
        );
    }

    #[test]
    fn windows_active_directory_double_slash_in_groupname() {
        let row = r"-rw-r--r--   1 300794   AD\\Domain Users     6148 Sep 19 06:17 .DS_Store";

        let ftpfile = FtpFile::try_from(row);
        assert!(ftpfile.is_ok());
        let ftpfile = ftpfile.unwrap();

        assert_eq!(ftpfile.entity, FtpFileType::File);
        assert_eq!(ftpfile.name, ".DS_Store");
        assert_eq!(ftpfile.target, None);
        assert_eq!(ftpfile.sticky, false);
        assert_eq!(ftpfile.permissions, "rw-r--r--");
        assert_eq!(ftpfile.acl, false);
        assert_eq!(ftpfile.owner, "300794");
        assert_eq!(ftpfile.group, r"AD\\Domain Users");
        assert_eq!(ftpfile.size, 6148);
        assert_eq!(ftpfile.pointer, None);

        //TODO: checks for correct year
        assert_eq!(
            ftpfile.date.unwrap(),
            NaiveDate::from_ymd(ftpfile.date.unwrap().year(), 9, 19).and_hms(6, 17, 0)
        );
    }
}
