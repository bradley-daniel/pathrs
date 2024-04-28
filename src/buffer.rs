use std::io;

use crossterm::{
    cursor::MoveTo,
    style::{style, Color, Colors, PrintStyledContent, ResetColor, SetColors},
    terminal::{Clear, ClearType},
    QueueableCommand,
};

use crate::grid::Grid;
use crate::space::Space;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
        Self {
            cells: vec![Cell::default(); width * height],
            width,
            height,
        }
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
        Some(())
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
        Ok(())
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
        Ok(())
    }
}

// Assignment4_Tests Buffer
#[cfg(test)]
mod buffer_tests {
    use super::{Buffer, Cell};
    use crossterm::style::{Color, Colors};

    #[test]
    fn test_manual_put_buffer() {
        let width = 10;
        let height = 10;
        let mut buf = Buffer::new(width, height);
        let ch = 'X';
        let colors = Colors::new(Color::Cyan, Color::Magenta);
        let x = 5;
        let y = 5;

        buf.put(x, y, ch, colors);
        let expected = Cell { ch, colors };
        let acutal = buf.cells[buf.width * 5 + 5];

        assert_eq!(expected, acutal);
    }

    #[test]
    fn test_manual_puts_buffer() {
        let width = 10;
        let height = 10;
        let mut buf = Buffer::new(width, height);
        let ch = 'X';
        let colors = Colors::new(Color::Cyan, Color::Magenta);
        let x = 5;
        let y = 5;

        buf.put(x, y, ch, colors);
        let expected = Cell { ch, colors };
        let acutal = buf.cells[buf.width * 5 + 5];

        assert_eq!(expected, acutal);
    }
}

// Assignment4_Tests Buffer
#[cfg(test)]
mod input_space_partioning_buffer {
    // Input space partitioning
    use crossterm::style::{Color, Colors};

    use crate::buffer::*;

    #[test]
    fn test_input_space_base() {
        // F F
        let width = 10;
        let height = 10;
        let mut buf = Buffer::new(width, height);
        let ch = 'X';
        let colors = Colors::new(Color::Cyan, Color::Magenta);
        let x = 5;
        let y = 5;

        let result = buf.put(x, y, ch, colors);

        assert!(result.is_some());
    }

    // T F - Inviable 

    // T T
    #[test]
    fn test_input_space_1() {
        let width = 0;
        let height = 0;
        let mut buf = Buffer::new(width, height);
        let ch = 'X';
        let colors = Colors::new(Color::Cyan, Color::Magenta);
        let x = 5;
        let y = 5;

        let result = buf.put(x, y, ch, colors);

        assert!(result.is_none());
    }

    // F T - Invaible
}

// Assignment4_Tests Cell
#[cfg(test)]
mod cell_tests {
    use std::{sync::Arc, thread};

    use rand::{seq::SliceRandom, thread_rng};

    use crate::{point::Point, space::Space};

    use super::*;

    #[test]
    fn test_munual_from_space() {
        let fuzzy_test_num = 10000;

        let spaces = Arc::new([
            Space::Empty,
            Space::Obstacle,
            Space::End(Point::new(0, 0)),
            Space::Start(Point::new(0, 0)),
            Space::Path,
        ]);

        (0..fuzzy_test_num)
            .map(|_| {
                let spaces = spaces.clone();
                thread::spawn(move || {
                    let space = spaces.choose(&mut thread_rng()).unwrap();
                    let _other: Cell = Cell::from(*space);
                })
                .join()
                .ok()
            })
            .for_each(|value| assert_ne!(None, value));
    }
}
