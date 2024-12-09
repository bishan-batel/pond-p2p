use std::{
    io::{BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::net::message::ChatMessage;

pub struct Client;

impl Client {
    pub fn run(address: impl ToSocketAddrs, contents: String) -> eyre::Result<()> {
        let mut stream = BufWriter::new(TcpStream::connect(address)?);

        let msg = ChatMessage {
            contents,
            user: "Client".into(),
        };

        serde_json::to_writer(&mut stream, &msg)?;
        stream.flush()?;

        Ok(())
    }
}
