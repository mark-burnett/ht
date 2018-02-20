extern crate ansi_term;
extern crate clap;
extern crate failure;
extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod display;
mod opt;
mod theme;

use failure::Error;

fn main() {
    let options = opt::get_options();
    go(&options).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
}

fn go(options: &opt::Options) -> Result<(), Error> {
    let mut res = reqwest::get(&options.uri)?;
    let t = &theme::DEFAULT;

    display::header(&res, t)?;
    display::json(&mut res, t)?;
    Ok(())
}
