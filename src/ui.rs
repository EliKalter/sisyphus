use crate::app::{App, Tool};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    match app.current_tool {
        Tool::Welcome => {
            let p = Paragraph::new("Welcome Menu\n\nPress 'n' for Name tool\nPress 'q' to quit")
                .block(Block::default().title(" Sisyphus ").borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
        Tool::Name => {
            let p = Paragraph::new("Eli Kalter\n\nPress 'b' to go back")
                .block(Block::default().title(" Name Tool ").borders(Borders::ALL))
                .alignment(Alignment::Center);
            frame.render_widget(p, area);
        }
    }
}
