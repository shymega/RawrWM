extern crate clap;

use clap::{App, ArgMatches};

const VERSION: &str = env!("CARGO_PKG_VERSION");
fn parse_arguments() -> ArgMatches<'static> {
    App::new("RawrWM Daemon")
        .author("The RawrWM Developers")
        .about("Daemon for the RawrWM window manager")
        .version(VERSION)
        .get_matches()
}

fn main() {
    let _matches = parse_arguments();
}
