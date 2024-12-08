use app::App;
use clap::Parser;
use crossterm::{
    event::{self, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{self, enable_raw_mode, EnterAlternateScreen},
};
use net::server::Server;
use ratatui::{prelude::Backend, Terminal};
use rayon::slice::ParallelSliceMut;

pub mod net;

mod app;
mod pond;
mod ui;

struct SimpleWrapper<T: ?Sized>(T);

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), anyhow::Error> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        match key.code {
            KeyCode::Char('q') => return Ok(()),
            _ => {}
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "false")]
    server: bool,

    #[arg(short, long)]
    port: Option<String>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    if args.server {
        Server::run();
    }

    let mut a = [420, 2, 4, 2];

    a.sort();

    a.par_sort();

    Ok(())
    // let mut terminal = ratatui::init();
    // terminal.clear()?;
    //
    // let mut app = App::new();
    //
    // let result = run_app(&mut terminal, &mut app);
    // ratatui::restore();
    //
    // result
}
