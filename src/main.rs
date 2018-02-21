#![feature(nll)]

extern crate ansi_term;
extern crate clap;
extern crate failure;
extern crate libc;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod display;
mod opt;
mod theme;

use failure::Error;

fn main() {
    std::process::exit(match go() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    })
}

fn go() -> Result<(), Error> {
    let options = opt::get_options()?;

    let mut stdout = std::io::stdout();

    let req = reqwest::Request::new(options.method, options.url);
    if options.display_req_headers {
        display::request_path(&mut stdout, &req, options.theme)?;
        display::header(&mut stdout, req.headers(), options.theme)?;
    }

    if options.display_req_bodies {
        // Awkwardly, might need to do this before building the request.
        unimplemented!()
    }

    let mut res = reqwest::Client::new().execute(req)?;

    if options.display_res_headers {
        display::response_status(&mut stdout, &res.status(), options.theme)?;
        display::header(&mut stdout, res.headers(), options.theme)?;
    }

    if options.display_res_bodies {
        if options.format_bodies {
            display::formatted_response(&mut stdout, &mut res, options.theme)?;
        } else {
            display::unformatted_response(&mut stdout, &mut res, options.theme)?;
        }
    }

    Ok(())
}
