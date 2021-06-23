/*
kiss-ftpd: an FTP server that Keeps It Simple, Stupid
Copyright (C) 2021  Travis Burtrum

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::env::{args_os, current_dir};
use unftp_sbe_fs::ServerExt;

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    #[cfg(feature = "env_logger")]
    {
        use env_logger::{Builder, Env, Target};
        let env = Env::default().filter_or("KISS_FTPD_LOG_LEVEL", "warn").write_style_or("KISS_FTPD_LOG_STYLE", "never");
        let mut builder = Builder::from_env(env);
        builder.target(Target::Stdout);
        builder.init();
    }

    let mut args = args_os().skip(1);

    let default_ftp_host = "0.0.0.0:21";

    let first_arg = args.next().unwrap_or_else(|| current_dir().expect("no access to current working dir, send in path").into_os_string());
    let ftp_host = args.next().map_or_else(|| default_ftp_host.to_owned(), |a| a.into_string().expect("invalid bind_address"));

    if first_arg == "-h" || first_arg == "--help" {
        println!(
            r#"usage: {} [options...] [ftp_directory] [bind_address]
 -h, --help                      print this usage text
 -V, -v, --version               Show version number then quit

 If ftp_directory not specified, defaults to current working directory
 If bind_address not specified, defaults to {}
        "#,
            env!("CARGO_PKG_NAME"),
            default_ftp_host,
        );
        return;
    } else if first_arg == "-V" || first_arg == "-v" || first_arg == "--version" {
        println!("{} {} ", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }

    println!("kiss-ftp listening on {} serving directory: {:?}", ftp_host, first_arg);

    let server = libunftp::Server::with_fs(first_arg).greeting("Welcome to kiss-ftpd");

    server.listen(ftp_host).await.expect("could not bind to address");
}
