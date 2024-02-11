#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Obstacle,
    Empty,
    Visited,
    Path,
    Start(Point),
    End(Point),
}

impl Space {}

pub struct Grid {
    pub spaces: Vec<Space>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        return Self {
            spaces: vec![Space::Empty; width * height],
            width,
            height,
        };
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Space> {
        let index = y * self.width + x;
        if x >= self.width || y >= self.height {
            None
        } else {
            self.spaces.get(index)
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Space> {
        let index = y * self.width + x;
        if x >= self.width || y >= self.height {
            None
        } else {
            self.spaces.get_mut(index)
        }
    }

    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        if y >= self.height || x >= self.width {
            None
        } else {
            Some(y * self.width + x)
        }
    }

    pub fn clear(&mut self) -> &Self {
        for space in self.spaces.iter_mut() {
            *space = Space::Empty;
        }
        return self;
    }
}
