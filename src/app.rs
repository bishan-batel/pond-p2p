use std::collections::HashMap;
use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::widgets::block::Title;
use ratatui::{prelude::*, widgets::Block};
use ratatui::{widgets::Paragraph, DefaultTerminal};
use symbols::border;

enum Main {}

#[derive(Debug)]
pub struct App {
    key: String,
    value: String,
    pairs: HashMap<String, String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            key: String::new(),
            value: String::new(),
            pairs: HashMap::new(),
        }
    }
}
