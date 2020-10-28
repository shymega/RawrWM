extern crate clap;

use clap::{
    App,
    ArgMatches,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_arguments() -> ArgMatches<'static> {
    App::new("rawrwmc")
        .author("The RawrWM Developers")
        .version(VERSION)
        .get_matches()
}

fn main() {
    let _matches = parse_arguments();
    unimplemented!();
}
