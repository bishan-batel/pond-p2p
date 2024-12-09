use std::fs::File;

use clap::Parser;
use net::{client::Client, server::Server};

pub mod net;

mod app;
mod pond;
mod ui;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    /// Run instance as a server
    #[arg(short, long, default_value = "false")]
    server: bool,

    /// Specifies a specific address to bind to
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    port: String,

    #[arg(short, long)]
    message: Option<String>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    fern::Dispatch::new()
        .chain(File::create(if args.server {
            "server.log"
        } else {
            "client.log"
        })?)
        .apply()?;

    if args.server {
        Server::run(args.port)
    } else {
        Client::run(args.port, args.message.unwrap_or("Default Message".into()))
    }
}
