use crossterm::style::{Color, Colors};

use crate::buffer::Cell;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

// impl Point {
//     fn new(x: usize, y: usize) -> Self {
//         Self { x, y }
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Obstacle,
    Empty,
    Visited,
    Start(Point),
    End(Point),
}

impl Space {
}

pub struct Grid {
    pub grid: Vec<Space>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            grid: vec![Space::Empty; width * height],
            width,
            height,
        };
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Space> {
        let index = y * self.width + x;
        self.grid.get(index)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Space> {
        let pos = y * self.width + x;
        self.grid.get_mut(pos)
    }

    pub fn clear(&mut self) -> &Self {
        for space in self.grid.iter_mut() {
            *space = Space::Empty;
        }
        return self;
    }

    // pub fn with_start(&mut self, x: usize, y: usize) -> Option<&Self> {
    //     let space = self.get_mut(x, y)?;
    //     let point = Point::new(x, y);
    //     *space = Space::Start(point);
    //     return Some(self);
    // }
    //
    // pub fn with_end(&mut self, x: usize, y: usize) -> Option<&Self> {
    //     let space = self.get_mut(x, y)?;
    //     let point = Point::new(x, y);
    //     *space = Space::End(point);
    //     return Some(self);
    // }
}
