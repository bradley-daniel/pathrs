#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn index(&self, width: usize) -> usize {
        self.y * width + self.x
    }

    pub fn in_bound(&self, width: usize, height: usize) -> bool {
        self.x >= width || self.y >= height
    }
}
