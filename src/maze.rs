use std::{
    collections::VecDeque,
    ops::Add,
    sync::{Arc, Mutex}, thread, time::Duration,
};

use crate::grid::{Grid, Point, Space};
use rand::Rng;

pub enum Orientation {
    Horz,
    Vert,
}

pub struct RandomMaze {
    pub grid: Arc<Mutex<Grid>>,
    pub start: Point,
}

impl RandomMaze {
    pub fn new(grid: Arc<Mutex<Grid>>) -> Self {
        Self {
            grid,
            start: Point::default(),
        }
    }

    fn randomize_start(&mut self) -> Point {
        let mut rng = rand::thread_rng();
        let mut grid = self.grid.lock().unwrap();
        loop {
            let x = rng.gen_range(0..grid.width);
            let y = rng.gen_range(0..grid.height);
            if let Some(Space::Empty) = grid.get(x, y) {
                let point = Point { x, y };
                *grid.get_mut(x, y).unwrap() = Space::Start(point);
                return point;
            }
        }
    }

    fn randomize_end(&mut self) -> Point {
        let mut rng = rand::thread_rng();
        let mut grid = self.grid.lock().unwrap();
        loop {
            let x = rng.gen_range(0..grid.width);
            let y = rng.gen_range(0..grid.height);
            if let Some(Space::Empty) = grid.get(x, y) {
                let point = Point { x, y };
                *grid.get_mut(x, y).unwrap() = Space::End(point);
                return point;
            }
        }
    }

    pub fn build_maze(&mut self) {
        {
            let mut grid = self.grid.lock().unwrap();
            grid.clear();
            let mut rng = rand::thread_rng();
            for space in grid.grid.iter_mut() {
                if rng.gen_bool(0.2) {
                    *space = Space::Obstacle;
                }
            }
        }
        self.start = self.randomize_start();
        self.randomize_end();
    }
}

fn get_adjacent(current: Point, grid: &Grid) -> VecDeque<Point> {
    let x = current.x as i64;
    let y = current.y as i64;
    vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
        .into_iter()
        .filter_map(|cord| {
            let x: Result<usize, _> = cord.0.try_into();
            let y: Result<usize, _> = cord.1.try_into();
            if let (Ok(x), Ok(y)) = (x, y) {
                Some(Point { x, y })
            } else {
                None
            }
        })
        .into_iter()
        .filter_map(|point| {
            if let Some(Space::Empty | Space::End(_)) = grid.get(point.x, point.y) {
                Some(point)
            } else {
                None
            }
        })
        .collect()
}

pub fn bfs(start: Point, grid: Arc<Mutex<Grid>>) {
    let mut queue = VecDeque::from([start]);
    'outer: while !queue.is_empty() {
        thread::sleep(Duration::from_millis(15));
        let current = queue.pop_front().unwrap();
        let mut data = grid.lock().unwrap();
        let mut adjacents = get_adjacent(current, &data);
        for point in &adjacents {
            if let Some(Space::End(_)) = data.get(point.x, point.y) {
                break 'outer;
            } else {
                *data.get_mut(point.x, point.y).unwrap() = Space::Visited;
            }
        }
        queue.append(&mut adjacents);
    }
}
