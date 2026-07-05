mod app;
// use ratatui::{DefaultTerminal, Frame};
//
use crate::app::App;

fn main() -> anyhow::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))?;

    Ok(())
}
