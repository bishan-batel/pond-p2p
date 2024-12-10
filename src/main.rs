#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![allow(clippy::missing_errors_doc)]

use core::str;
use std::{default, fs::File, io::stdout, sync::Arc};

use bytes::Bytes;
use clap::Parser;
use futures::{SinkExt, StreamExt, TryStreamExt};
use log::{error, info};
use net::{
    protocol,
    server::{self, Server},
    user::Username,
};
use rustyline::error::ReadlineError;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite, LinesCodec};

pub mod net;

mod app;
mod pond;
mod ui;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Specifies a specific address to bind to
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    address: String,

    #[command(subcommand)]
    cmd: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    #[command(name = "host", subcommand_value_name = "[POND NAME]")]
    Server { name: String },

    #[command(name = "join")]
    Client { user_name: String },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.cmd {
        Commands::Client { user_name } => {
            fern::Dispatch::new()
                .chain(File::create("client.log")?)
                .apply()?;

            let mut server = TcpStream::connect(&args.address).await?;

            let (reader, writer) = server.split();
            let mut stream = FramedRead::new(reader, BytesCodec::new());
            let mut sink = FramedWrite::new(writer, BytesCodec::new());

            sink.send(Bytes::from(ron::to_string(&protocol::RegisterRequest {
                user: user_name.clone().into(),
            })?))
            .await?;

            match ron::from_str::<protocol::RegisterResponse>(str::from_utf8(
                &stream.next().await.unwrap()?,
            )?)? {
                protocol::RegisterResponse::Ok => {}
                protocol::RegisterResponse::Denied { reason } => {
                    error!("Denied connection: {reason}");
                    println!("Denied connection: {reason}");
                    return Ok(());
                }
            }

            let mut rl = rustyline::DefaultEditor::new()?;

            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str())?;

                        println!("[{user_name}] {line}");
                        sink.send(Bytes::from(ron::to_string(
                            &server::Request::SendMessage { message: line },
                        )?))
                        .await?;
                    }
                    Err(err) => {
                        error!("Error: {err:?}");
                        break;
                    }
                    _ => {
                        break;
                    }
                }
            }

            Ok(())
        }
        Commands::Server { name } => {
            fern::Dispatch::new()
                .chain(File::create("out.log")?)
                .apply()?;
            let server = Arc::new(Server::bind(&args.address).await?);

            // let main_handle = tokio::spawn({
            //     let server = server.clone();
            //     async move {
            //         let _ = server;
            //     }
            // });

            loop {
                let tcp = server.accept().await?;
                let server = server.clone();
                tokio::spawn(async move { server.handle_connection(tcp).await });
            }

            // main_handle.await?;
            // Ok(())
        }
    }
}
