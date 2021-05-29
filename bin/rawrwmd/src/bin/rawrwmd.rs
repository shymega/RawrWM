extern crate clap;
extern crate jsonrpc_ipc_server as rpc;
#[macro_use]
extern crate serde_derive;

use clap::{App, ArgMatches};
use rpc::jsonrpc_core::*;
use rpc::ServerBuilder;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Deserialize)]
struct WindowCloseParams {
    window_uuid: String,
}

fn parse_arguments() -> ArgMatches<'static> {
    App::new("RawrWM Daemon")
        .author("The RawrWM Developers")
        .about("Daemon for the RawrWM window manager")
        .version(VERSION)
        .get_matches()
}

fn main() {
    let _matches = parse_arguments();

    let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").unwrap();
    let process_pid = std::process::id();
    let full_sock_dir =
        format!("{}/rawrwm/control-{}.sock", xdg_runtime_dir, process_pid)
            .to_owned();

    let mut io = IoHandler::new();

    io.add_sync_method("wm/system/version", |_params| {
        println!("wm/system/version method called...");

        Ok(Value::String(VERSION.to_owned()))
    });

    io.add_sync_method("wm/window/close", |params: Params| {
        println!("wm/window/close method called...");

        let parsed: WindowCloseParams =
            params.parse().expect("No method parameters sent.");

        let window_uuid = parsed.window_uuid;

        Ok(Value::String(format!("Window: {} - closed.", window_uuid)))
    });

    let builder = ServerBuilder::new(io);

    let server = builder
        .start(&full_sock_dir)
        .expect("Could not open IPC socket, bailing.");

    server.wait();
}
