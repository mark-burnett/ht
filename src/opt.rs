use clap::{App, Arg};
use failure::Error;
use libc;
use std;
use std::os::unix::io::AsRawFd;
use theme;

#[derive(Debug)]
pub struct Options<'a> {
    pub theme: &'a theme::Theme,
    pub uri: String,
    pub format_bodies: bool,
    pub display_res_headers: bool,
    pub display_res_bodies: bool,
    pub display_req_headers: bool,
    pub display_req_bodies: bool,
}

pub fn get_options<'a>() -> Result<Options<'a>, Error> {
    let matches = App::new("ht")
        .arg(Arg::with_name("uri").required(true))
        .get_matches();

    let uri = matches
        .value_of("uri")
        .expect("Failed to unwrap url")
        .to_string();

    let out_is_tty = stdout_is_tty();
    let t: &theme::Theme = if out_is_tty {
        &theme::DEFAULT
    } else {
        &theme::EMPTY
    };

    Ok(Options {
        uri: uri,
        theme: t,
        format_bodies: true,
        display_res_headers: true,
        display_res_bodies: true,
        display_req_headers: false,
        display_req_bodies: false,
    })
}

fn stdout_is_tty() -> bool {
    unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 }
}
