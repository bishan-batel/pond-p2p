use core::str;
use std::{
    io::{self, BufRead, BufReader, Read},
    net::{TcpListener, ToSocketAddrs},
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::Duration,
};

use bytes::BytesMut;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use log::{error, info};
use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, List},
    DefaultTerminal,
};

use super::message::ChatMessage;

#[derive(Debug)]
pub struct Server {
    rx: Receiver<ChatMessage>,
    handle: JoinHandle<eyre::Result<()>>,
}

impl Server {
    fn init(address: impl ToSocketAddrs) -> Result<Self, io::Error> {
        let stream = TcpListener::bind(address)?;

        let (tx, rx) = mpsc::channel();

        info!("Starting server");

        let handle = thread::spawn(move || {
            for conn in stream.incoming() {
                info!("Stream");

                let stream = match conn {
                    Ok(stream) => stream,
                    Err(err) => {
                        error!("Server {:#?}", err);
                        return Err(err.into());
                    }
                };

                let reader = BufReader::new(stream);

                match serde_json::from_reader(reader) {
                    Ok(msg) => {
                        info!("Sending Message: {:?}", msg);
                        tx.send(msg)?
                    }
                    Err(err) => {
                        error!("Decode Error: {}", err);
                        return Err(err)?;
                    }
                }
            }
            Ok(())
        });

        Ok(Self { handle, rx })
    }

    pub fn main_loop(self, mut terminal: DefaultTerminal) -> eyre::Result<()> {
        let mut messages = vec![];

        while !self.handle.is_finished() {
            match self.rx.try_recv() {
                Ok(msg) => {
                    info!("Received message {:#?}", msg);
                    messages.push(msg);
                }
                Err(err) => match err {
                    mpsc::TryRecvError::Empty => {}
                    mpsc::TryRecvError::Disconnected => {
                        error!("Thread disconnected!");

                        return Err(err.into());
                    }
                },
            }

            if let Err(err) = terminal.draw(|frame| {
                // render frame

                let block = Block::new()
                    .title("Server")
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().red())
                    .borders(Borders::all());

                let list = List::new(
                    messages
                        .iter()
                        .map(|x| format!("{}: {}", x.user, x.contents)),
                )
                .repeat_highlight_symbol(true)
                .highlight_symbol(">>")
                .block(block);

                frame.render_widget(list, frame.area());
            }) {
                error!("Error rendering frame: {}", err);
            }

            if !event::poll(Duration::from_secs(0))? {
                continue;
            }

            match event::read()? {
                Event::Key(key) => match key {
                    KeyEvent {
                        kind: KeyEventKind::Press,
                        code: KeyCode::Char('q'),
                        ..
                    } => return Ok(()),
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    pub fn run(address: impl ToSocketAddrs) -> eyre::Result<()> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        let result = Self::init(address)?.main_loop(terminal);
        ratatui::restore();
        result
    }
}

#[cfg(test)]
mod test {

    #[test]
    pub fn test1() {
        let a = "huh";

        let b = " bruh  ";

        assert_eq!(a, b);
    }
}
