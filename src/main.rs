pub mod buffer;
pub mod grid;
pub mod maze;
pub mod screen_state;

use crossterm::terminal;
use maze::bfs;

use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::{io::stdout, sync::Arc};

use buffer::{Buffer, Cell};
use screen_state::ScreenState;

fn main() -> std::io::Result<()> {
    let _screen_state = ScreenState::enable()?;

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ScreenState::disable();
        default_hook(info);
    }));

    let mut stdout = stdout();
    let (width, height) = terminal::size()?;

    let grid = grid::Grid::new(width as usize, height as usize);
    let grid = Arc::new(Mutex::new(grid));

    let mut buf = Buffer::new(width as usize, height as usize);

    let mut maze = maze::RandomMaze::new(grid);
    maze.build_maze();
    let start = maze.start;

    let thread_grid = maze.grid.clone();
    let maze_thread = thread::spawn(move || {
        bfs(start, thread_grid);
    });

    // let grid = maze.grid.lock().unwrap();
    // for (space, cell) in grid.grid.iter().zip(buf.cells.iter_mut()) {
    //     *cell = Cell::from(space);
    // }
    // buf.flush(&mut stdout)?;
    // drop(grid);

    while !maze_thread.is_finished() {
        std::thread::sleep(Duration::from_millis(150));
        let grid = maze.grid.lock().unwrap();
        for (space, cell) in grid.grid.iter().zip(buf.cells.iter_mut()) {
            *cell = Cell::from(space);
        }
        buf.flush(&mut stdout)?;
    }

    return Ok(());
}
