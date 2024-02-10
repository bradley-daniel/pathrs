use std::io::{self, stdout};

use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

pub struct ScreenState;

impl ScreenState {
    pub fn enable() -> io::Result<Self> {
        crossterm::execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        return Ok(Self);
    }
}

impl Drop for ScreenState {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode().map_err(|err| {
            eprintln!("ERROR: disable_raw_mode: {err}");
        });
        let _ = crossterm::execute!(stdout(), LeaveAlternateScreen).map_err(|err| {
            eprintln!("ERROR: LeaveAlternateScreen: {err}");
        });
    }
}
