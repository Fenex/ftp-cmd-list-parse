mod unix {
    use std::convert::TryFrom;

    use ::ftp_cmd_list_parse::*;

    #[test]
    fn normal_directory() {
        let row = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "usr");
        assert_eq!(ftpentry.size(), 4096);
        assert_eq!(ftpentry.date_str(), "Dec 21 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 12, 21).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxr-xr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn normal_directory_2() {
        let row = "drwxrwxrwx   1 owner   group          0 Aug 31 2012 e-books";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "e-books");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Aug 31 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 8, 31).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrwx");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn normal_file() {
        let row = "-rw-rw-rw-   1 owner   group    7045120 Sep 02  2012 music.mp3";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "music.mp3");
        assert_eq!(ftpentry.size(), 7045120);
        assert_eq!(ftpentry.date_str(), "Sep 02 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw-rw-");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_number_id_owner() {
        let row = "-rw-rw-rw-   1 1234   group    7045120 Sep 02  2012 music.mp3";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "music.mp3");
        assert_eq!(ftpentry.size(), 7045120);
        assert_eq!(ftpentry.date_str(), "Sep 02 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw-rw-");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "1234");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_number_id_group() {
        let row = "-rw-rw-rw-   1 owner   1234    7045120 Sep 02  2012 music.mp3";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "music.mp3");
        assert_eq!(ftpentry.size(), 7045120);
        assert_eq!(ftpentry.date_str(), "Sep 02 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw-rw-");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "1234");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_space_in_group() {
        let row =
            "-rwxrwxr-x    1 1317       Domain Use                3065 May  4 11:01 xmlrpc.php";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "xmlrpc.php");
        assert_eq!(ftpentry.size(), 3065);
        assert_eq!(ftpentry.date_str(), "May 4 11:01");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 4).and_hms(11, 1, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "1317");
        assert_eq!(ftpentry_unix.group, "Domain Use");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_double_space_in_group() {
        let row =
            "-rwxrwxr-x    1 1317       Domain  Use                3065 May  4 11:01 xmlrpc.php";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "xmlrpc.php");
        assert_eq!(ftpentry.size(), 3065);
        assert_eq!(ftpentry.date_str(), "May 4 11:01");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 4).and_hms(11, 1, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "1317");
        assert_eq!(ftpentry_unix.group, "Domain  Use");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_space_in_owner_name() {
        let row = "-rwxrwxr-x    1 Domain Use       33                3065 May  4 11:01 xmlrpc.php";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "xmlrpc.php");
        assert_eq!(ftpentry.size(), 3065);
        assert_eq!(ftpentry.date_str(), "May 4 11:01");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 4).and_hms(11, 1, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "Domain Use");
        assert_eq!(ftpentry_unix.group, "33");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_double_space_in_owner_name() {
        let row =
            "-rwxrwxr-x    1 Domain  Use       33                3065 May  4 11:01 xmlrpc.php";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "xmlrpc.php");
        assert_eq!(ftpentry.size(), 3065);
        assert_eq!(ftpentry.date_str(), "May 4 11:01");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 4).and_hms(11, 1, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "Domain  Use");
        assert_eq!(ftpentry_unix.group, "33");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_number_owner_and_hyphen_groupname() {
        let row = "-rw-------    1 33         www-data           14 May 15 01:52 .ftpquota";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), ".ftpquota");
        assert_eq!(ftpentry.size(), 14);
        assert_eq!(ftpentry.date_str(), "May 15 01:52");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 15).and_hms(1, 52, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-------");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "33");
        assert_eq!(ftpentry_unix.group, "www-data");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_hyphen_owner_and_number_groupname() {
        let row = "-rw-------    1 www-data         33           14 May 15 01:52 .ftpquota";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), ".ftpquota");
        assert_eq!(ftpentry.size(), 14);
        assert_eq!(ftpentry.date_str(), "May 15 01:52");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 15).and_hms(1, 52, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-------");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "www-data");
        assert_eq!(ftpentry_unix.group, "33");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn file_with_acl_set() {
        let row = "-rw-rw-rw-+   1 owner   group    7045120 Sep 02  2012 music.mp3";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "music.mp3");
        assert_eq!(ftpentry.size(), 7045120);
        assert_eq!(ftpentry.date_str(), "Sep 02 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw-rw-");
        assert_eq!(ftpentry_unix.acl, true);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_sticky_bit_and_executable_for_others() {
        let row = "drwxrwxrwt   7 root   root    4096 May 19 2012 tmp";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "tmp");
        assert_eq!(ftpentry.size(), 4096);
        assert_eq!(ftpentry.date_str(), "May 19 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, true);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrwx");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_sticky_bit_and_executable_for_others_2() {
        let row = "drwxrwx--t   7 root   root    4096 May 19 2012 tmp";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "tmp");
        assert_eq!(ftpentry.size(), 4096);
        assert_eq!(ftpentry.date_str(), "May 19 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, true);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwx--x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_sticky_bit_and_not_executable_for_others() {
        let row = "drwxrwxrwT   7 root   root    4096 May 19 2012 tmp";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "tmp");
        assert_eq!(ftpentry.size(), 4096);
        assert_eq!(ftpentry.date_str(), "May 19 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, true);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrw-");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_sticky_bit_and_not_executable_for_others_2() {
        let row = "drwxrwx--T   7 root   root    4096 May 19 2012 tmp";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "tmp");
        assert_eq!(ftpentry.size(), 4096);
        assert_eq!(ftpentry.date_str(), "May 19 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 5, 19).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, true);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwx---");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_space_in_group_name() {
        let row = "drwxrwxr-x    7 1317       Domain Use        208 May  5 11:28 wp-content";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "wp-content");
        assert_eq!(ftpentry.size(), 208);
        assert_eq!(ftpentry.date_str(), "May 5 11:28");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 5).and_hms(11, 28, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "1317");
        assert_eq!(ftpentry_unix.group, "Domain Use");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_double_space_in_group_name() {
        let row = "drwxrwxr-x    7 1317       Domain  Use        208 May  5 11:28 wp-content";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "wp-content");
        assert_eq!(ftpentry.size(), 208);
        assert_eq!(ftpentry.date_str(), "May 5 11:28");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 5).and_hms(11, 28, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "1317");
        assert_eq!(ftpentry_unix.group, "Domain  Use");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_space_in_group_name_and_owner_name() {
        let row = "drwxrwxr-x    7 Domain Use       Domain Use        208 May  5 11:28 wp-content";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "wp-content");
        assert_eq!(ftpentry.size(), 208);
        assert_eq!(ftpentry.date_str(), "May 5 11:28");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 5, 5).and_hms(11, 28, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "Domain Use");
        assert_eq!(ftpentry_unix.group, "Domain Use");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_undeifned_bit_state() {
        let row = "drwxr-S---    3 105207   501            18 Jul 04  2017 .pki";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), ".pki");
        assert_eq!(ftpentry.size(), 18);
        assert_eq!(ftpentry.date_str(), "Jul 04 2017");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxr-S---");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "105207");
        assert_eq!(ftpentry_unix.group, "501");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    #[allow(non_snake_case)]
    fn directory_with_set_the_setUserID_or_setGroupID_bit() {
        let row = "drwxr-s---    3 105207   501            18 Jul 04  2017 .pki";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), ".pki");
        assert_eq!(ftpentry.size(), 18);
        assert_eq!(ftpentry.date_str(), "Jul 04 2017");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxr-s---");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "105207");
        assert_eq!(ftpentry_unix.group, "501");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_mandatory_lock() {
        let row = "drwx--L---    3 105207   501            18 Jul 04  2017 .pki";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), ".pki");
        assert_eq!(ftpentry.size(), 18);
        assert_eq!(ftpentry.date_str(), "Jul 04 2017");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwx--L---");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "105207");
        assert_eq!(ftpentry_unix.group, "501");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_block_device_simple() {
        let row = "brwx-w----    3 105207   501            18 Jul 04  2017 .pki";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::BlockDevice);
        assert_eq!(ftpentry.name(), ".pki");
        assert_eq!(ftpentry.size(), 18);
        assert_eq!(ftpentry.date_str(), "Jul 04 2017");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwx-w----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "105207");
        assert_eq!(ftpentry_unix.group, "501");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_block_device() {
        let row = "brw-rw----  1 root disk    8,   0 Nov 24 10:13 sda";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::BlockDevice);
        assert_eq!(ftpentry.name(), "sda");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Nov 24 10:13");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 11, 24).and_hms(10, 13, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "disk");
        assert_eq!(ftpentry_unix.pointer, Some("8,0".to_string()));
    }

    #[test]
    fn directory_with_character_device_without_pointer() {
        let row = "crw-rw----  1 root tty       0 Apr  1 20:30 vcs";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::CharacterDevice);
        assert_eq!(ftpentry.name(), "vcs");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Apr 1 20:30");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 4, 1).and_hms(20, 30, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "tty");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_character_device_with_pointer() {
        let row = "crw-rw---- 1 root tty       7, 134 Apr  1 20:30 vcsa6";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::CharacterDevice);
        assert_eq!(ftpentry.name(), "vcsa6");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Apr 1 20:30");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 4, 1).and_hms(20, 30, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "tty");
        assert_eq!(ftpentry_unix.pointer, Some("7,134".to_string()));
    }

    #[test]
    fn directory_with_character_device() {
        let row = "crw-rw----  1 root tty       7,   0 Apr  1 20:30 vcs";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::CharacterDevice);
        assert_eq!(ftpentry.name(), "vcs");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Apr 1 20:30");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 4, 1).and_hms(20, 30, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "tty");
        assert_eq!(ftpentry_unix.pointer, Some("7,0".to_string()));
    }

    #[test]
    fn directory_with_named_pipe() {
        let row = "prwx-w----    3 105207   501            18 Jul 04  2017 .pki";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Pipe);
        assert_eq!(ftpentry.name(), ".pki");
        assert_eq!(ftpentry.size(), 18);
        assert_eq!(ftpentry.date_str(), "Jul 04 2017");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwx-w----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "105207");
        assert_eq!(ftpentry_unix.group, "501");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_socket() {
        let row = "srwx-w----    3 105207   501            18 Jul 04  2017 .pki";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Socket);
        assert_eq!(ftpentry.name(), ".pki");
        assert_eq!(ftpentry.size(), 18);
        assert_eq!(ftpentry.date_str(), "Jul 04 2017");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2017, 7, 4).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwx-w----");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "105207");
        assert_eq!(ftpentry_unix.group, "501");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_symlink() {
        let row = "lrwxrwxrwx 1 root root 51 Apr  4 23:57 www.nodeftp.github -> /etc/nginx/sites-available/www.nodeftp.github";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Symlink);
        assert_eq!(ftpentry.name(), "www.nodeftp.github");
        assert_eq!(ftpentry.size(), 51);
        assert_eq!(ftpentry.date_str(), "Apr 4 23:57");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 4, 4).and_hms(23, 57, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(
            ftpentry_unix.target,
            Some("/etc/nginx/sites-available/www.nodeftp.github".to_string())
        );
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrwx");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn macos_special_symbol_file() {
        let row = "-rw-rw-rw-@   1 owner   group    7045120 Sep 02  2012 music.mp3";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "music.mp3");
        assert_eq!(ftpentry.size(), 7045120);
        assert_eq!(ftpentry.date_str(), "Sep 02 2012");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 9, 2).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-rw-rw-");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn directory_with_special_name_2() {
        let row = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 1.1 Header [13]";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "1.1 Header [13]");
        assert_eq!(ftpentry.size(), 4096);
        assert_eq!(ftpentry.date_str(), "Dec 21 2012");
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(2012, 12, 21).and_hms(0, 0, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxr-xr-x");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "root");
        assert_eq!(ftpentry_unix.group, "root");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn windows_with_unix_style_directory_1() {
        let row = "drwxrwxrwx   1 owner    group               0 Aug 22 14:05 wwwroot";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "wwwroot");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Aug 22 14:05");
        //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 8, 22).and_hms(14, 5, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrwx");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn windows_with_unix_style_directory_2() {
        let row = "drwxrwxrwx   1 owner    group               0 Aug 22 14:05 Name []";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "Name []");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "Aug 22 14:05");
        // //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 8, 22).and_hms(14, 5, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrwx");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn windows_with_unix_style_file() {
        let row = "-rwxrwxrwx   1 owner    group           99710 Aug 22 12:59 iisstart.png";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "iisstart.png");
        assert_eq!(ftpentry.size(), 99710);
        assert_eq!(ftpentry.date_str(), "Aug 22 12:59");

        // //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 8, 22).and_hms(12, 59, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rwxrwxrwx");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "owner");
        assert_eq!(ftpentry_unix.group, "group");
        assert_eq!(ftpentry_unix.pointer, None);
    }

    #[test]
    fn windows_active_directory_double_slash_in_groupname() {
        let row = r"-rw-r--r--   1 300794   AD\\Domain Users     6148 Sep 19 06:17 .DS_Store";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), ".DS_Store");
        assert_eq!(ftpentry.size(), 6148);
        assert_eq!(ftpentry.date_str(), "Sep 19 06:17");
        // //TODO: checks for correct year
        // assert_eq!(
        //     ftpentry.date(),
        //     NaiveDate::from_ymd(ftpentry.date().year(), 9, 19).and_hms(6, 17, 0)
        // );

        assert_eq!(ftpentry.is_msdos_type(), false);
        assert_eq!(ftpentry.is_unix_type(), true);

        let ftpentry_unix = ftpentry.to_unix_type();

        assert_eq!(ftpentry_unix.target, None);
        assert_eq!(ftpentry_unix.sticky, false);
        assert_eq!(ftpentry_unix.permissions.as_str(), "rw-r--r--");
        assert_eq!(ftpentry_unix.acl, false);
        assert_eq!(ftpentry_unix.owner, "300794");
        assert_eq!(ftpentry_unix.group, r"AD\\Domain Users");
        assert_eq!(ftpentry_unix.pointer, None);
    }
}
