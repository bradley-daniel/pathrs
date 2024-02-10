use std::time::Duration;

use rand::Rng;

use crate::grid::{Grid, Space};

// pub trait Maze {
//     fn draw(&mut self) -> Grid;
// }
//
pub enum Orientation {
    Horz,
    Vert,
}

pub struct RandomMaze<'a> {
    grid: &'a mut Grid,
}

impl<'a> RandomMaze<'a> {
    pub fn new(grid: &'a mut Grid) -> Self {
        Self { grid }
    }

    pub fn draw_wall(&mut self, start: usize, end: usize, direction: Orientation) {
        todo!();
    }

    pub fn build_maze(&mut self) {
        self.grid.clear();
        let mut h = self.grid.height;
        let mut w = self.grid.width;
        if w % 2 == 0 {
            w -= 1;
        }
        if h % 2 == 0 {
            h -= 1;
        }

        let mut rng = rand::thread_rng();
        for space in self.grid.grid.iter_mut() {
            if rng.gen_bool(0.2) {
                *space = Space::Obstacle;
            }
        }
    }
    fn divide(&mut self, width: usize, height: usize) {
        if (width < 2 || height < 2) {
            return;
        };

        let (x_begin, y_begin, oirientation) = self.randomize_wall(width, height);
    }

    fn randomize_wall(&mut self, width: usize, height: usize) -> (usize, usize, Orientation) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..(width / 2) - 1);
        let y = rng.gen_range(0..(height / 2) - 1);
        // let t_height = (height - 1) / 2;
        // let t_width = (width - 1) / 2;
        return (0, 0, Orientation::Horz);
    }
}
