use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    buffer::Buffer,
    layout::{Constraint::Length, Layout, Rect},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default, PartialEq, Eq)]
enum AppMode {
    #[default]
    Running,
    Quit,
}

#[derive(Default)]
pub struct App {
    mode: AppMode,
}

impl App {
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        while self.is_running() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;

            if let Event::Key(key) = event::read()?
                && key.code == KeyCode::Char('q')
            {
                self.mode = AppMode::Quit
            }
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode == AppMode::Running
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // todo!()
        Paragraph::new("Hello")
            .block(Block::bordered().title("nakku"))
            .render(area, buf);
    }
}
