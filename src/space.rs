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

impl Space {
    pub fn is_pathable(&self) -> bool {
        matches!(self, Space::End(_) | Space::Empty)
    }
}

