use core::str;
use std::{collections::HashMap, sync::Arc};

use bytes::Bytes;
use eyre::OptionExt;
use futures::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
};
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite, LinesCodec};

use super::{
    protocol::{self, RegisterRequest, ServerRequest},
    user::{User, Username},
};

pub type Request = protocol::ServerRequest;
pub type Response = protocol::ServerResponse;
pub type ResponseError = protocol::ServerResponseError;

pub struct Server {
    root_tcp: TcpListener,
}

#[derive(Debug, Clone, Hash)]
pub enum BroadcastMessage {
    Message { user: Username, contents: String },
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum ClientMessage {}

impl Server {
    /// # Errors
    ///
    /// This function will return an error if it was unable to listen on the address
    pub async fn bind(addr: impl tokio::net::ToSocketAddrs) -> eyre::Result<Self> {
        let tcp = TcpListener::bind(addr).await?;

        // let (tx, rx) = broadcast::channel(16);

        Ok(Server { root_tcp: tcp })
    }

    pub async fn accept(&self) -> eyre::Result<TcpStream> {
        Ok(self.root_tcp.accept().await?.0)
    }

    pub async fn handle_connection(&self, mut tcp: TcpStream) -> eyre::Result<()> {
        let (reader, writer) = tcp.split();

        let mut stream = FramedRead::new(reader, BytesCodec::new());

        let mut sink = FramedWrite::new(writer, BytesCodec::new());

        let user = {
            let msg = stream.next().await.ok_or_eyre("Huh")??;
            let msg = match str::from_utf8(&msg) {
                Ok(msg) => msg,
                Err(err) => {
                    error!("Failed to parse client packet: {}", err);
                    return Err(err.into());
                }
            };

            ron::from_str::<RegisterRequest>(msg)?.user
        };

        sink.send(Bytes::from(ron::to_string(
            &protocol::RegisterResponse::Ok,
        )?))
        .await?;
        println!("{user} Joined");

        while let Some(Ok(msg)) = stream.next().await {
            let msg = match str::from_utf8(&msg) {
                Ok(msg) => msg,
                Err(err) => {
                    error!("Failed to parse client packet: {}", err);
                    continue;
                }
            };

            let msg = match ron::from_str::<ServerRequest>(msg) {
                Ok(msg) => msg,
                Err(err) => {
                    error!("Failed to parse client packet: {}", err);
                    continue;
                }
            };

            match msg {
                ServerRequest::SendMessage { message } => {
                    info!("{user}: {message}");
                    println!("{user}: {message}");
                }
            }
        }

        Ok(())
    }
}
