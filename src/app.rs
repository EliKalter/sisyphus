pub enum Tool {
    Welcome,
    Name,
}

pub struct App {
    pub current_tool: Tool,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tool: Tool::Welcome,
            should_quit: false,
        }
    }
}
