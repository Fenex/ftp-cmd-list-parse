mod msdos;
mod unix;

use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
    ops::Deref,
};

pub use msdos::FtpEntryMsdos;
pub use unix::FtpEntryUnix;

/// Permissions of the Unix-like entry.
#[derive(Debug)]
pub struct FtpEntryPermissions(String);

impl FtpEntryPermissions {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for FtpEntryPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

/// Type of the ftp entry.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FtpEntryKind {
    UNKNOWN,
    Directory,
    File,
    BlockDevice,
    CharacterDevice,
    Pipe,
    Socket,
    Symlink,
    // Symlink(FtpEntryPath)
}

impl From<char> for FtpEntryKind {
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

impl TryFrom<&str> for FtpEntryKind {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() == 1 {
            Ok(value.chars().nth(0).unwrap().into())
        } else {
            Err("length of the value must be equal to 1")
        }
    }
}

/// All fields that supports both servers: Unix & MSDOS
pub trait FtpEntryInfo {
    /// Returns a new `FtpEntry` by given string if parsing was successful.
    /// Also you can create new `FtpEntry` by use `TryFrom` trait.
    /// ```rust
    /// # use ftp_cmd_list_parse::FtpEntry;
    /// let ftp_response = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
    ///
    /// match FtpEntry::new(ftp_response) {
    ///     Some(ftp_entry) => {
    ///         assert_eq!(ftp_entry.name(), "usr");
    ///         assert_eq!(ftp_entry.size(), 4096);
    ///         assert_eq!(ftp_entry.date_str(), "Dec 21 2012");
    ///     }
    ///     None => println!("ftp_response is not valid ftp-entry!")
    /// }
    /// ```
    fn new(string: &str) -> Option<Self>
    where
        for<'a> Self: TryFrom<&'a str>,
    {
        Self::try_from(string).ok()
    }

    /// Represents type of the entry: Directory, File, Symlink or other.
    fn kind(&self) -> FtpEntryKind;
    /// Returns name of the entry.
    fn name(&self) -> &str;
    /// Returns size of the entry.
    fn size(&self) -> usize;
    // fn date(&self) -> NaiveDateTime;
    /// Returns date of the entry.
    fn date_str(&self) -> &str;
}

/// Represents parsed string as ftp entry.
///
/// Implements `Deref` to `&dyn FtpEntryInfo`, so you can get access
/// to general fields that supports both servers: Unix & MSDOS.
#[derive(Debug)]
pub enum FtpEntry {
    Unix(FtpEntryUnix),
    Msdos(FtpEntryMsdos),
}

impl FtpEntry {
    /// Returns a new `FtpEntry` by given string if parsing was successful.
    /// Also you can create new `FtpEntry` by use `TryFrom` or `TryInto` traits.
    /// ```rust
    /// # use ftp_cmd_list_parse::FtpEntry;
    /// let ftp_response = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
    ///
    /// match FtpEntry::new(ftp_response) {
    ///     Some(ftp_entry) => {
    ///         assert_eq!(ftp_entry.name(), "usr");
    ///         assert_eq!(ftp_entry.size(), 4096);
    ///         assert_eq!(ftp_entry.date_str(), "Dec 21 2012");
    ///     }
    ///     None => println!("ftp_response is not valid ftp-entry!")
    /// }
    /// ```
    pub fn new(string: &str) -> Option<Self> {
        string.try_into().ok()
    }

    /// Returns true if `FtpEntry` has UNIX-like entry, otherwise false.
    pub fn is_unix_type(&self) -> bool {
        match self {
            FtpEntry::Unix(_) => true,
            _ => false,
        }
    }

    /// Returns true if `FtpEntry` has MSDOS-like entry, otherwise false.
    pub fn is_msdos_type(&self) -> bool {
        match self {
            FtpEntry::Msdos(_) => true,
            _ => false,
        }
    }

    /// Converts `FtpEntry` to `FtpEntryUnix`.
    /// Its may be useful if you need to get additional infomation
    /// like permissions, group, owner and others.
    ///
    /// # Panics
    ///
    /// Panics if the value is not a Unix-like entry.
    /// If you not sure what kind of FtpEntry is, use `try_to_unix_type` instead.
    pub fn to_unix_type(self) -> FtpEntryUnix {
        self.try_to_unix_type().expect("FtpEntryType missmatch")
    }

    /// Converts `FtpEntry` to `FtpEntryMsdos`.
    ///
    /// # Panics
    ///
    /// Panics if the value is not a Msdos-like entry.
    /// If you not sure what kind of FtpEntry is, use `try_to_msdos_type` instead.
    pub fn to_msdos_type(self) -> FtpEntryMsdos {
        self.try_to_msdos_type().expect("FtpEntryType missmatch")
    }

    /// Tries to convert `FtpEntry` to `FtpEntryUnix`.
    /// If it is impossible, returns `FtpEntry `back to caller inside `Err`.
    pub fn try_to_unix_type(self) -> Result<FtpEntryUnix, Self> {
        if let FtpEntry::Unix(entry) = self {
            Ok(entry)
        } else {
            Err(self)
        }
    }

    /// Tries to convert `FtpEntry` to `FtpEntryMsdos`.
    /// If it is impossible, returns `FtpEntry `back to caller inside `Err`.
    pub fn try_to_msdos_type(self) -> Result<FtpEntryMsdos, Self> {
        if let FtpEntry::Msdos(entry) = self {
            Ok(entry)
        } else {
            Err(self)
        }
    }
}

impl Deref for FtpEntry {
    type Target = dyn FtpEntryInfo;

    fn deref(&self) -> &Self::Target {
        match self {
            FtpEntry::Msdos(entry) => entry,
            FtpEntry::Unix(entry) => entry,
        }
    }
}

impl TryFrom<&str> for FtpEntry {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(entry) = FtpEntryUnix::try_from(value) {
            return Ok(FtpEntry::Unix(entry));
        }

        if let Ok(entry) = FtpEntryMsdos::try_from(value) {
            return Ok(FtpEntry::Msdos(entry));
        }

        Err(())
    }
}
