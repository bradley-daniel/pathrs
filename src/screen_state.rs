use std::io::{self, stdout};

use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};

pub struct ScreenState;

impl ScreenState {
    pub fn enable() -> io::Result<Self> {
        crossterm::execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            ScreenState::disable();
            default_hook(info);
        }));

        return Ok(Self);
    }
    pub fn disable() {
        let _ = terminal::disable_raw_mode().map_err(|err| {
            eprintln!("ERROR: disable_raw_mode: {err}");
        });
        let _ = crossterm::execute!(stdout(), LeaveAlternateScreen).map_err(|err| {
            eprintln!("ERROR: LeaveAlternateScreen: {err}");
        });
    }
}

impl Drop for ScreenState {
    fn drop(&mut self) {
        Self::disable();
    }
}
