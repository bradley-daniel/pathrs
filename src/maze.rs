use std::{
    collections::VecDeque,
    fs,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
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
        let mut grid = self.grid.lock().unwrap();
        grid.clear();
        let mut rng = rand::thread_rng();
        for space in grid.spaces.iter_mut() {
            if rng.gen_bool(0.2) {
                *space = Space::Obstacle;
            }
        }
        drop(grid);
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
    let len = grid.lock().unwrap().spaces.len();
    let mut queue = VecDeque::from([start]);
    let mut pred = vec![0; len];

    let mut end = 0;

    'outer: while !queue.is_empty() {
        thread::sleep(Duration::from_millis(2));
        let current = queue.pop_front().unwrap();
        let mut data = grid.lock().unwrap();
        let mut adjacents = get_adjacent(current, &data);
        let parent_index = data.index(current.x, current.y).unwrap();

        for adjacent in &adjacents {
            let adjacent_index = data.index(adjacent.x, adjacent.y).unwrap();
            pred[adjacent_index] = parent_index;
            if let Some(Space::End(_)) = data.get(adjacent.x, adjacent.y) {
                // println!("end: {}", adjacent_index);
                end = adjacent_index;
                break 'outer;
            } else {
                *data.get_mut(adjacent.x, adjacent.y).unwrap() = Space::Visited;
            }
        }
        queue.append(&mut adjacents);
    }
    let mut data = grid.lock().unwrap();
    let mut path: Vec<usize> = Vec::new();

    let mut crawl = pred[end];
    while !matches!(data.spaces[crawl], Space::Start(_)) {
        path.push(crawl);
        crawl = pred[crawl];
    }
    drop(data);

    for value in path {
        thread::sleep(Duration::from_millis(15));
        let mut data = grid.lock().unwrap();
        data.spaces[value] = Space::Path;
    }
}
