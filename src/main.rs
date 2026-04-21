use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::Paragraph,
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let text = Paragraph::new("Eli Kalter");
            frame.render_widget(text, frame.area());
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
