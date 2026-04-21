use crate::app::{App, Tool};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) {
    // We only care about press events to avoid double triggers
    if key_event.kind != KeyEventKind::Press {
        return;
    }

    match (key_event.code, &app.current_tool) {
        // Global quit command
        (KeyCode::Char('q'), _) => app.should_quit = true,

        // Welcome screen navigation
        (KeyCode::Char('n'), Tool::Welcome) => app.current_tool = Tool::Name,

        // Name tool navigation
        (KeyCode::Char('b'), Tool::Name) => app.current_tool = Tool::Welcome,

        _ => {}
    }
}
