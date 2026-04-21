mod app;
mod keyboard_handler;
mod tui;
mod ui;

use app::App;
use ratatui::crossterm::event::{self, KeyEventKind};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let mut app = App::new();
    let res = run(&mut terminal, &mut app);
    tui::restore()?;
    res
}

fn run(terminal: &mut tui::Tui, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|f| ui::render(f, app))?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    keyboard_handler::handle_key_events(key, app);
                }
            }
        }
    }
    Ok(())
}
