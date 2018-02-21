extern crate ansi_term;
extern crate clap;
extern crate failure;
extern crate libc;
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

    let mut res = reqwest::get(&options.uri)?;

    let mut stdout = std::io::stdout();

    if options.display_res_headers {
        display::header(&mut stdout, &res, options.theme)?;
    }

    if options.format_bodies {
        display::json(&mut stdout, &mut res, options.theme)?;
    } else {
        res.copy_to(&mut stdout)?;
    }

    Ok(())
}
