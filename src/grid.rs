use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn into_index(&self, width: usize) -> usize {
        self.y * width + self.x
    }

    fn in_bound(&self, width: usize, height: usize) -> bool {
        self.x >= width || self.y >= height
    }
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

// impl Space {}

pub struct Grid {
    pub spaces: Vec<Space>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            spaces: vec![Space::Empty; width * height],
            width,
            height,
        }
    }

    pub fn get(&self, point: Point) -> Option<Space> {
        let index = point.into_index(self.width);
        // let index = point.y * self.width + point.x;
        // if x >= self.width || y >= self.height {
        //     None
        // } else {
        self.spaces.get(index).copied()
        // }
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut Space> {
        let index = point.into_index(self.width);
        self.spaces.get_mut(index)
    }

    pub fn index(&self, point: Point) -> Option<usize> {
        if point.in_bound(self.width, self.height) {
            None
        } else {
            Some(point.into_index(self.width))
        }
    }

    pub fn clear(&mut self) -> &Self {
        for space in self.spaces.iter_mut() {
            *space = Space::Empty;
        }
        self
    }

    pub fn adjacents_points(&self, point: Point) -> Vec<Point> {
        let x = point.x;
        let y = point.y;
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(|direction| {
                let x = x.checked_add_signed(direction.0);
                let y = y.checked_add_signed(direction.1);
                if let (Some(x), Some(y)) = (x, y) {
                    match self.get(Point { x, y }) {
                        Some(_) => Some(Point { x, y }),
                        None => None,
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
