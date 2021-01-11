## ftp-cmd-list-parse

This is a library that can parse strings that represents answers from FTP servers by `LIST` command request.

```
drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr
brw-rw----  1 root disk    8,   0 Nov 24 10:13 sda
-rw-rw-rw-   1 owner   1234    7045120 Sep 02  2012 music.mp3
lrwxrwxrwx 1 root root 51 Apr  4 23:57 www.nodeftp.github -> /etc/nginx/sites-available/www.nodeftp.github
```

Currently supports UNIX-style only. MSDOS-style is coming soon.

Example of use:

```Rust
let ftp_response: &'static str = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
if let Some(ftp_entry) = FtpEntry::new(ftp_response) {
    println!("{}", ftp_entry.name()); // usr
    println!("{:?}", ftp_entry.kind()); // FtpEntryKind::Directory

    assert!(ftp_entry.is_unix_type()); // true
}
```

You need convert `FtpEntry` to `FtpEntryUnix ` to see additional fields that MSDOS FTP server doesn't support:

```Rust
    // also you can create `FtpEntry` by use `TryFrom` trait.
    use std::convert::TryFrom;
    use ftp_cmd_list_parse::FtpEntry;

    let ftp_response: &'static str = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";

    if let Ok(ftp_entry) = FtpEntry::try_from(ftp_response) {
        match ftp_entry.try_to_unix_type() {
            Ok(ftp_entry_unix) => { // `FtpEntryUnix` type
                println!("Owner: {}", ftp_entry_unix.owner);
                println!("group: {}", ftp_entry_unix.group);
                println!("Permissions: {}", ftp_entry_unix.permissions.as_str());
            },
            Err(ftp_entry) => { // `FtpEntry` type
                // Here we got our `FtpEntry` back.
                println!("FtpEntry is not a UNIX-format!");
            }
        }
    }
```

If you ensure that you work with UNIX FTP server, you can create `FtpEntryUnix` struct directly (Or `FtpEntryMsdos` in the future):

```rust
use ftp_cmd_list_parse::FtpEntryUnix;
let ftp_response: &'static str = "drwxr-xr-x  10 root   root    4096 Dec 21  2012 usr";
if let Some(ftp_entry_unix) = FtpEntryUnix::new(ftp_response) {
    println!("Owner: {}", ftp_entry_unix.owner);
    println!("group: {}", ftp_entry_unix.group);
    println!("Permissions: {}", ftp_entry_unix.permissions);
}
```