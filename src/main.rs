pub mod buffer;
pub mod grid;
pub mod maze;
pub mod point;
pub mod screen_state;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal;
use maze::{bfs, RandomMaze};

use std::sync::Mutex;
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::{io::stdout, sync::Arc};

use buffer::Buffer;
use screen_state::ScreenState;

fn main() -> std::io::Result<()> {
    let _screen_state = ScreenState::enable()?;

    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ScreenState::disable();
        default_hook(info);
    }));

    loop {
        let (width, height) = terminal::size()?;
        let grid = grid::Grid::new(width as usize, height as usize);

        // let mut buf = Buffer::new(width as usize, height as usize);
        let grid = Arc::new(Mutex::new(grid));

        let mut maze = maze::RandomMaze::new(grid);
        maze.build_maze();
        let start = maze.start;

        let thread_grid = maze.grid.clone();
        let maze_thread = thread::spawn(move || {
            bfs(start, thread_grid);
        });

        let writer_thread = thread::spawn(move || {
            let _ = writer_thread(maze_thread, maze);
        });

        // let _ = writer_thread.join();

        if poll(Duration::from_millis(15))? {
            if let Event::Key(event) = read()? {
                if event.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        let _ = writer_thread.join();
    }
    drop(_screen_state);
    Ok(())
}

fn writer_thread(maze_thread: JoinHandle<()>, maze: RandomMaze) -> std::io::Result<()> {
    let mut stdout = stdout();
    let (width, height) = terminal::size()?;
    let mut buf = Buffer::new(width as usize, height as usize);
    while !maze_thread.is_finished() {
        std::thread::sleep(Duration::from_millis(15));
        let grid = maze.grid.lock().unwrap();
        let _ = buf.flush_diff(&mut stdout, &grid);
    }
    thread::sleep(Duration::from_secs(2));
    let _ = maze_thread.join();
    Ok(())
}
