use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    Obstacle,
    Empty,
    Visited,
    Path,
    Start(Point),
    End(Point),
}

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
        let index = point.index(self.width);
        self.spaces.get(index).copied()
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut Space> {
        let index = point.index(self.width);
        self.spaces.get_mut(index)
    }

    pub fn index(&self, point: Point) -> Option<usize> {
        if point.in_bound(self.width, self.height) {
            None
        } else {
            Some(point.index(self.width))
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
                    self.get(Point { x, y }).map(|_| Point { x, y })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}
