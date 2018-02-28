/**
 *    Copyright 2018 Mark Burnett
 *
 *    Licensed under the Apache License, Version 2.0 (the "License");
 *    you may not use this file except in compliance with the License.
 *    You may obtain a copy of the License at
 *
 *        http://www.apache.org/licenses/LICENSE-2.0
 *
 *    Unless required by applicable law or agreed to in writing, software
 *    distributed under the License is distributed on an "AS IS" BASIS,
 *    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *    See the License for the specific language governing permissions and
 *    limitations under the License.
 */
use clap::{App, Arg};
use failure::Error;
use libc;
use reqwest;
use std;
use std::os::unix::io::AsRawFd;
use theme;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Options<'a> {
    pub theme: &'a theme::Theme,
    pub url: reqwest::Url,
    pub method: reqwest::Method,
    pub format_bodies: bool,
    pub display_res_headers: bool,
    pub display_res_bodies: bool,
    pub display_req_headers: bool,
    pub display_req_bodies: bool,
}

pub fn get_options<'a>() -> Result<Options<'a>, Error> {
    let matches = App::new("ht")
        .arg(
            Arg::with_name("url")
                .required(true)
                .validator(|ref val| validate_url(val)),
        )
        .arg(
            Arg::with_name("print")
                .long("print")
                .takes_value(true)
                .validator(|ref val| validate_print(val))
                .help("Specify what parts of requests/responses to display"),
        )
        .get_matches();

    let url = reqwest::Url::parse(
        matches
            .value_of("url")
            .expect("Failed to get url from matches"),
    )?;

    let method = reqwest::Method::Get;

    let out_is_tty = stdout_is_tty();
    let t: &theme::Theme = if out_is_tty {
        &theme::DEFAULT
    } else {
        &theme::EMPTY
    };

    let mut display_req_bodies = false;
    let mut display_req_headers = false;
    let mut display_res_bodies = true;
    // Only display response headeres by default if the output is a tty.
    let mut display_res_headers = out_is_tty;

    if let Some(print) = matches.value_of("print") {
        display_req_bodies = print.contains('B');
        display_req_headers = print.contains('H');
        display_res_bodies = print.contains('b');
        display_res_headers = print.contains('h');
    }

    Ok(Options {
        url: url,
        method: method,
        theme: t,
        format_bodies: true, // For now at least, disabling body formatting also disables themes on them.
        display_res_headers: display_res_headers,
        display_res_bodies: display_res_bodies,
        display_req_headers: display_req_headers,
        display_req_bodies: display_req_bodies,
    })
}

fn stdout_is_tty() -> bool {
    unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 }
}

fn validate_print(val: &str) -> Result<(), String> {
    let r = Regex::from_str(r"^[BHbh]{1,4}$").expect("Failed to parse print regex");
    if r.is_match(val) {
        Ok(())
    } else {
        Err("valid values are: BHbh".to_string())
    }
}

fn validate_url(val: &str) -> Result<(), String> {
    match reqwest::Url::parse(val) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
