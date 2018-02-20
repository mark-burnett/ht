use clap::{App, Arg};

#[derive(Debug)]
pub struct Options {
    pub uri: String,
}

pub fn get_options() -> Options {
    let matches = App::new("ht")
        .arg(Arg::with_name("uri").required(true))
        .get_matches();
    Options {
        uri: matches
            .value_of("uri")
            .expect("Failed to unwrap url")
            .to_string(),
    }
}
