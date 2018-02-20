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
    let t = &theme::DEFAULT;

    let mut stdout = std::io::stdout();
    display::header(&mut stdout, &res, t)?;
    display::json(&mut stdout, &mut res, t)?;
    Ok(())
}
