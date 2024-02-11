use std::io;

use crossterm::{
    cursor::MoveTo,
    style::{style, Color, Colors, PrintStyledContent, ResetColor, SetColors},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::grid::{Grid, Space};

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Cell {
    pub ch: char,
    pub colors: Colors,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            colors: Colors {
                foreground: Some(Color::White),
                background: None,
            },
        }
    }
}

impl From<Space> for Cell {
    fn from(value: Space) -> Self {
        match value {
            Space::Obstacle => Cell {
                ch: ' ',
                colors: Colors {
                    foreground: None,
                    background: Some(Color::DarkRed),
                },
            },
            Space::Empty => Cell {
                ch: ' ',
                colors: Colors {
                    foreground: None,
                    background: None,
                },
            },
            Space::Visited => Cell {
                ch: 'O',
                colors: Colors {
                    foreground: Some(Color::White),
                    background: None,
                },
            },
            Space::Path => Cell {
                ch: ' ',
                colors: Colors {
                    foreground: None,
                    background: Some(Color::AnsiValue(7)),
                },
            },
            Space::Start(_) => Cell {
                ch: ' ',
                colors: Colors {
                    foreground: None,
                    background: Some(Color::DarkBlue),
                },
            },
            Space::End(_) => Cell {
                ch: ' ',
                colors: Colors {
                    foreground: None,
                    background: Some(Color::DarkGreen),
                },
            },
        }
    }
}

impl From<&Space> for Cell {
    fn from(value: &Space) -> Self {
        Self::from(*value)
    }
}

pub struct Buffer {
    pub cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            cells: vec![Cell::default(); width * height],
            width,
            height,
        };
    }

    pub fn put(&mut self, x: usize, y: usize, ch: char, colors: Colors) -> Option<()> {
        let pos = y * self.width + x;
        if let Some(buffer_cell) = self.cells.get_mut(pos) {
            *buffer_cell = Cell { ch, colors };
            Some(())
        } else {
            None
        }
    }

    pub fn puts(&mut self, x: usize, y: usize, chars: &[char], colors: Colors) -> Option<()> {
        let start = y * self.width + x;
        if start + chars.len() > self.width || y > self.height {
            return None;
        }
        for (cell, &ch) in self.cells.iter_mut().zip(chars.iter()) {
            *cell = Cell { ch, colors }
        }
        return Some(());
    }

    pub fn flush(&self, wrte: &mut impl io::Write) -> io::Result<()> {
        wrte.queue(Clear(ClearType::All))?;
        for &Cell { ch, colors } in self.cells.iter() {
            let styled_content = style(ch);
            wrte.queue(SetColors(colors))?;
            wrte.queue(PrintStyledContent(styled_content))?;
            wrte.queue(ResetColor)?;
        }
        wrte.flush()?;
        return Ok(());
    }

    pub fn flush_diff(&mut self, wrte: &mut impl io::Write, grid: &Grid) -> io::Result<()> {
        for (i, (space, cell)) in grid.spaces.iter().zip(self.cells.iter_mut()).enumerate() {
            let x = (i % grid.width).try_into().unwrap();
            let y = (i / grid.width).try_into().unwrap();
            let new_cell = Cell::from(space);
            if new_cell != *cell {
                *cell = new_cell;
                let (ch, colors) = (cell.ch, cell.colors);
                let styled_content = style(ch);
                wrte.queue(MoveTo(x, y))?;
                wrte.queue(SetColors(colors))?;
                wrte.queue(PrintStyledContent(styled_content))?;
                wrte.queue(ResetColor)?;
            }
        }
        wrte.flush()?;
        return Ok(());
    }
}
