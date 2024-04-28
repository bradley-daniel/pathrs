use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::grid::{Grid, Space};
use crate::point::Point;
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
            let point = Point { x, y };
            if let Some(Space::Empty) = grid.get(point) {
                *grid.get_mut(point).unwrap() = Space::Start(point);
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
            let point = Point { x, y };
            if let Some(Space::Empty) = grid.get(point) {
                *grid.get_mut(point).unwrap() = Space::End(point);
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

pub fn bfs(start: Point, grid: Arc<Mutex<Grid>>) {
    let len = grid.lock().unwrap().spaces.len();
    let mut queue = VecDeque::from([start]);
    let mut pred = vec![0; len];

    let mut end = 0;

    'outer: while !queue.is_empty() {
        thread::sleep(Duration::from_millis(2));
        let current = queue.pop_front().unwrap();
        let mut data = grid.lock().unwrap();
        let mut empty_adj: VecDeque<Point> = data
            .adjacents_points(current)
            .into_iter()
            .filter(|point| match data.get(*point) {
                Some(s) => s.is_pathable(),
                _ => false,
            })
            .collect();

        let parent_index = data.unchecked_index(current);

        for adjacent in &empty_adj {
            let adjacent_index = data.unchecked_index(*adjacent);
            pred[adjacent_index] = parent_index;
            if let Some(Space::End(_)) = data.get(*adjacent) {
                end = adjacent_index;
                break 'outer;
            } else {
                *data.get_mut(*adjacent).unwrap() = Space::Visited;
            }
        }
        queue.append(&mut empty_adj);
    }

    let data = grid.lock().unwrap();
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
