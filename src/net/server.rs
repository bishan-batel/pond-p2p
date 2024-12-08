use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{prelude::Backend, Terminal};

#[derive(Debug)]
pub struct Server {}

impl Server {
    fn init() -> Self {
        Self {}
    }

    pub fn main_loop<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), anyhow::Error> {
        todo!("bruh");
        panic!("huh");
        loop {
            let Event::Key(key) = event::read()? else {
                continue;
            };

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }

    pub fn run() -> Result<(), anyhow::Error> {
        let mut terminal = ratatui::init();
        let result = Self::init().main_loop(&mut terminal);
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
