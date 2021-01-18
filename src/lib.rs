//! # ftp-cmd-list-parse
//!
//! This is a library that can parse strings that FTP servers return
//! by `LIST` command request.
//!
//! * Unix-style:
//! ```text
//! drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr
//! brw-rw----  1 root disk    8,   0 Nov 24 10:13 sda
//! -rw-rw-rw-   1 owner   1234    7045120 Sep 02  2012 music.mp3
//! lrwxrwxrwx 1 root root 51 Apr  4 23:57 www.nodeftp.github -> /etc/nginx/sites-available/www.nodeftp.github
//! ```
//!
//! * Msdos-style:
//! ```text
//! 08-22-2018  02:05PM       <DIR>          wwwroot
//! 08-22-18  12:59PM                99710 logo.jpg
//! 08-22-18  03:01AM                99710 music.mp3
//! ```
//!
//! ## Examples:
//!
//! ```rust
//! use ftp_cmd_list_parse::FtpEntry;
//!
//! let ftp_response: &'static str = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
//! if let Some(ftp_entry) = FtpEntry::new(ftp_response) {
//!     println!("{}", ftp_entry.name()); // "usr"
//!     println!("{:?}", ftp_entry.kind()); // FtpEntryKind::Directory
//!
//!     assert!(ftp_entry.is_unix_type()); // true
//! }
//! ```
//!
//! You need convert `FtpEntry` to `FtpEntryUnix` to see additional
//! fields that MSDOS FTP server doesn't support:
//!
//! ```rust
//! use std::convert::TryFrom;
//! use ftp_cmd_list_parse::FtpEntry;
//!
//! let ftp_response: &'static str = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
//!
//! if let Ok(ftp_entry) = FtpEntry::try_from(ftp_response) {
//!     match ftp_entry.try_to_unix_type() {
//!         Ok(ftp_entry_unix) => { // `FtpEntryUnix` type
//!             println!("Owner: {}", ftp_entry_unix.owner); // "root"
//!             println!("Group: {}", ftp_entry_unix.group); // "root"
//!             println!("Permissions: {}", ftp_entry_unix.permissions.as_str()); // "rwxr-xr-x"
//!         },
//!         Err(ftp_entry) => { // `FtpEntry` type
//!             // Here we got our `FtpEntry` back.
//!             println!("FtpEntry is not an UNIX-format!");
//!         }
//!     }
//!
//!     // Also you can use pattern-matching to destruct enum:
//!     // if let FtpEntry::Msdos(ftp_entry_msdos) = ftp_entry {
//!     //    println!("name: {}", ftp_entry_msdos.name()); // prints "name: usr"
//!     //}
//! }
//! ```
//!
//! If you ensure what type of FTP server using, you can create `FtpEntryUnix`
//! or `FtpEntryMsdos` struct directly:
//!
//! ```rust
//! use ftp_cmd_list_parse::FtpEntryUnix;
//!
//! let ftp_response: &'static str = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
//! if let Some(ftp_entry_unix) = FtpEntryUnix::new(ftp_response) {
//!     println!("Owner: {}", ftp_entry_unix.owner); // "root"
//!     println!("Group: {}", ftp_entry_unix.group); // "root"
//!     println!("Permissions: {}", ftp_entry_unix.permissions); // "rwxr-xr-x"
//! }
//! ```

#[macro_use]
extern crate lazy_static;

mod entry;
pub use entry::*;
