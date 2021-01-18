mod unix {
    use std::convert::TryFrom;

    use ::ftp_cmd_list_parse::*;

    #[test]
    fn normal_directory() {
        let row = "08-22-18  02:05PM       <DIR>          Test";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "Test");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2018-08-22T14:05");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_directory_2_lowercase_ampm() {
        let row = "08-22-18  02:05pm       <DIR>          Name []";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "Name []");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2018-08-22T14:05");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_directory_with_2_digit_characters_in_filename() {
        let row = "08-22-18  02:05PM       <DIR>          12";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "12");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2018-08-22T14:05");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_directory_with_4_digit_characters_in_filename() {
        let row = "08-22-18  02:05PM       <DIR>          2015";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "2015");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2018-08-22T14:05");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_directory_3() {
        let row = "08-22-18  02:05PM       <DIR>          1.1 Header [13]";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "1.1 Header [13]");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2018-08-22T14:05");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_directory_with_four_digit_year() {
        let row = "08-22-2018  02:05PM       <DIR>          wwwroot";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "wwwroot");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2018-08-22T14:05");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_directory_without_tabs() {
        let row = "07-10-13 06:54AM <DIR> 1400";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::Directory);
        assert_eq!(ftpentry.name(), "1400");
        assert_eq!(ftpentry.size(), 0);
        assert_eq!(ftpentry.date_str(), "2013-07-10T06:54");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_file() {
        let row = "08-22-18  12:59PM                99710 iisstart.png";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "iisstart.png");
        assert_eq!(ftpentry.size(), 99710);
        assert_eq!(ftpentry.date_str(), "2018-08-22T12:59");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_file_2() {
        let row = "08-22-18  12:59PM                990 2015";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "2015");
        assert_eq!(ftpentry.size(), 990);
        assert_eq!(ftpentry.date_str(), "2018-08-22T12:59");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }

    #[test]
    fn normal_file_3() {
        let row = "08-22-18  12:59PM                2015 2015";

        let ftpentry = FtpEntry::try_from(row);
        assert!(ftpentry.is_ok());
        let ftpentry = ftpentry.unwrap();

        assert_eq!(ftpentry.kind(), FtpEntryKind::File);
        assert_eq!(ftpentry.name(), "2015");
        assert_eq!(ftpentry.size(), 2015);
        assert_eq!(ftpentry.date_str(), "2018-08-22T12:59");
        assert_eq!(ftpentry.is_msdos_type(), true);
        assert_eq!(ftpentry.is_unix_type(), false);
    }
}
