kiss-ftpd
---------

[![Build Status](https://ci.moparisthe.best/job/moparisthebest/job/kiss-ftpd/job/master/badge/icon%3Fstyle=plastic)](https://ci.moparisthe.best/job/moparisthebest/job/kiss-ftpd/job/master/)
[![crates.io](https://img.shields.io/crates/v/kiss-ftpd.svg)](https://crates.io/crates/kiss-ftpd)

An FTP server that Keeps It Simple, Stupid.

It simply serves the specified directory over FTP, allowing reads and writes, and that's it!

If you are after a more complicated FTP server with all the bells and whistles, [unFTP](https://github.com/bolcom/unFTP) looks good.

##### Usage

```
usage: kiss-ftpd [options...] [ftp_directory] [bind_address]
 -h, --help                      print this usage text
 -V, -v, --version               Show version number then quit

 If ftp_directory not specified, defaults to current working directory
 If bind_address not specified, defaults to 0.0.0.0:21
```

There is an example systemd unit in `systemd/kiss-ftpd.service` which runs it with minimal permissions
and as locked down as possible.

Many thanks to [libunftp](https://github.com/bolcom/libunftp) which did all the heavy FTP lifting.
