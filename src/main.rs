use app::App;

pub mod app;

pub struct Server;

fn main() -> Result<(), anyhow::Error> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let app_result = App::run(terminal);
    ratatui::restore();

    app_result
}
