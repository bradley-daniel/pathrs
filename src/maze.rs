use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::point::Point;
use crate::{grid::Grid, space::Space};
use rand::{seq::IteratorRandom, Rng};

pub enum Orientation {
    Horz,
    Vert,
}

pub struct RandomMaze {
    pub grid: Arc<Mutex<Grid>>,
    pub start: Point,
    pub end: Point,
}

impl RandomMaze {
    pub fn new(grid: Arc<Mutex<Grid>>) -> Self {
        Self {
            grid,
            start: Point::default(),
            end: Point::default(),
        }
    }

    fn randomize_start(&mut self) -> Option<()> {
        let mut grid = self.grid.lock().unwrap();
        let mut rng = rand::thread_rng();

        let start = (0..grid.width * grid.height)
            .filter_map(|index| {
                let point = Point::from_index(index, grid.width);
                match grid.get(point)? {
                    Space::Empty => Some(point),
                    _ => None,
                }
            })
            .choose(&mut rng)?;

        *grid.get_mut(start).unwrap() = Space::Start(start);
        self.start = start;
        Some(())
    }

    fn randomize_end(&mut self) -> Option<()> {
        let mut grid = self.grid.lock().unwrap();
        let mut rng = rand::thread_rng();

        let end = (0..grid.width * grid.height)
            .filter_map(|index| {
                let point = Point::from_index(index, grid.width);
                match grid.get(point)? {
                    Space::Empty => Some(point),
                    _ => None,
                }
            })
            .choose(&mut rng)?;
        *grid.get_mut(end).unwrap() = Space::End(end);
        self.end = end;
        Some(())
    }

    fn randomize_obstacles(&mut self) {
        let mut grid = self.grid.lock().unwrap();
        grid.clear();
        let mut rng = rand::thread_rng();
        for space in grid.spaces.iter_mut() {
            if rng.gen_bool(0.2) {
                *space = Space::Obstacle;
            }
        }
    }

    pub fn build_maze(&mut self) -> Option<()> {
        self.randomize_obstacles();
        self.randomize_start()?;
        self.randomize_end()?;
        Some(())
    }
}

pub fn bfs(start: Point, grid: Arc<Mutex<Grid>>) -> Option<()> {
    let len = grid.lock().unwrap().spaces.len();
    let mut queue = VecDeque::from([start]);
    let mut pred = vec![0; len];

    let mut end = None;

    'outer: while !queue.is_empty() {
        thread::sleep(Duration::from_millis(1));
        let current = queue.pop_front().unwrap();
        let mut data = grid.lock().unwrap();
        let mut empty_adj: VecDeque<Point> = data
            .adjacent_points(current)
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
                end = Some(adjacent_index);
                break 'outer;
            } else {
                *data.get_mut(*adjacent).unwrap() = Space::Visited;
            }
        }
        queue.append(&mut empty_adj);
    }

    let data = grid.lock().unwrap();
    let mut path: Vec<usize> = Vec::new();

    let end = end?;
    let mut crawl = pred[end];
    while !matches!(data.spaces[crawl], Space::Start(_)) {
        path.push(crawl);
        crawl = pred[crawl];
    }

    drop(data);

    for value in path {
        thread::sleep(Duration::from_millis(10));
        let mut data = grid.lock().unwrap();
        data.spaces[value] = Space::Path;
    }
    Some(())
}

// Assignment4_Tests RadomMazeBuilder
#[cfg(test)]
mod random_maze_tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    use crate::grid::Grid;

    use super::RandomMaze;

    #[test]
    fn fuzzy_random_maze() {
        let fuzzy_test = 1000;
        (0..fuzzy_test)
            .map(|_| {
                thread::spawn(move || {
                    let width = 10;
                    let height = 10;
                    let grid = Arc::new(Mutex::new(Grid::new(width, height)));
                    let mut random_maze = RandomMaze::new(grid);
                    let _ = random_maze.build_maze();
                })
                .join()
                .ok()
            })
            .for_each(|results| {
                assert!(results.is_some());
            });
    }
}
