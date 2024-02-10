pub mod buffer;
pub mod grid;
pub mod maze;
pub mod screen_state;

use crossterm::{
    event::{poll, read, Event},
    style::{Color, Colors, Print},
    terminal::{self, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::io::stdout;
use std::time::Duration;

use buffer::{Buffer, Cell};
use screen_state::ScreenState;

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    let (width, height) = terminal::size()?;

    let mut maze = grid::Grid::new(width as usize, height as usize);
    // let _ = maze::RecursiveDivision::new(&mut maze).build_maze();
    let _ = maze::RandomMaze::new(&mut maze).build_maze();

    let _screen_state = ScreenState::enable()?;
    let mut buf = Buffer::new(width as usize, height as usize);

    buf.flush(&mut stdout)?;

    let _ = maze::RandomMaze::new(&mut maze).build_maze();
    let obstacle_colors = Colors {
        foreground: None,
        background: Some(Color::Red),
    };

    let space_colors = Colors {
        foreground: None,
        background: None,
    };
    for (space, cell) in maze.grid.iter().zip(buf.cells.iter_mut()) {
        match space {
            grid::Space::Obstacle => {
                *cell = Cell {
                    ch: ' ',
                    colors: obstacle_colors,
                }
            }
            grid::Space::Empty => {
                *cell = Cell {
                    ch: ' ',
                    colors: space_colors,
                }
            }
            // grid::Space::Visited => todo!(),
            _ => unreachable!("Should not be able to happen"),
        }
    }
    for _ in 0..1000 {
        let _ = maze::RandomMaze::new(&mut maze).build_maze();
        let obstacle_colors = Colors {
            foreground: None,
            background: Some(Color::Red),
        };

        let space_colors = Colors {
            foreground: None,
            background: None,
        };
        for (space, cell) in maze.grid.iter().zip(buf.cells.iter_mut()) {
            match space {
                grid::Space::Obstacle => {
                    *cell = Cell {
                        ch: ' ',
                        colors: obstacle_colors,
                    }
                }
                grid::Space::Empty => {
                    *cell = Cell {
                        ch: ' ',
                        colors: space_colors,
                    }
                }
                // grid::Space::Visited => todo!(),
                _ => unreachable!("Should not be able to happen"),
            }
        }
        std::thread::sleep(Duration::from_millis(15));
        buf.flush(&mut stdout)?;
    }
    return Ok(());
}
