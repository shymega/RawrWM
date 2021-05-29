extern crate clap;

use clap::{App, ArgMatches};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn parse_arguments() -> ArgMatches<'static> {
    App::new("RawrWM Client")
        .author("The RawrWM Developers")
        .about("IPC client to the RawrWM server")
        .version(VERSION)
        .get_matches()
}

fn main() {
    let _matches = parse_arguments();
    unimplemented!();
}
