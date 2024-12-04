use std::io;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::widgets::block::Title;
use ratatui::{prelude::*, widgets::Block};
use ratatui::{widgets::Paragraph, DefaultTerminal};
use symbols::border;

pub struct App {
    terminal: DefaultTerminal,
    should_close: bool,
}

pub trait Scene: Widget {
    fn title(&self) -> Title;
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // self.terminal.draw(|frame| {
        //     let block = Block::bordered().title("Pond").border_set(border::ROUNDED);
        //
        //     let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
        //         .white()
        //         .centered()
        //         .block(block);
        //
        //     frame.render_widget(greeting, frame.area());
        // })?;
    }
}
impl App {
    pub fn process(&mut self) -> Result<(), anyhow::Error> {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.should_close = true;
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn run(mut terminal: DefaultTerminal) -> Result<(), anyhow::Error> {
        let mut app = App {
            terminal,
            should_close: false,
        };

        while !app.should_close {
            app.process()?;
            app.terminal.draw(|frame| {
                frame.render_widget(&app, frame.area());
            })?;
        }

        Ok(())
    }
}
