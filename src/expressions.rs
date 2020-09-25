use regex::Regex;

lazy_static! {
    pub static ref REX_LISTUNIX: Regex = Regex::new(
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
    pub static ref REX_LISTMSDOS: Regex = Regex::new(
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
