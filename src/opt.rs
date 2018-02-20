use clap::{App, Arg};
use failure::Error;

#[derive(Debug)]
pub struct Options {
    pub uri: String,
}

pub fn get_options() -> Result<Options, Error> {
    let matches = App::new("ht")
        .arg(Arg::with_name("uri").required(true))
        .get_matches();
    Ok(Options {
        uri: matches
            .value_of("uri")
            .expect("Failed to unwrap url")
            .to_string(),
    })
}
