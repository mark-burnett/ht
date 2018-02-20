extern crate ansi_term;
extern crate clap;
extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod display;
mod opt;
mod theme;

fn main() {
    let options = opt::get_options();

    let mut res = reqwest::get(&options.uri).expect("Failed to fetch the thing");
    let t = &theme::DEFAULT;

    display::header(&res, t);
    display::json(&mut res, t);
}
