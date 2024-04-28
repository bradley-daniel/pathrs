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



// Assignment4_Tests Space
#[cfg(test)]
mod space_tests {
    use super::Space;

    #[test]
    fn manual_test_is_pathable() {
        let space = Space::Path;
        assert!(!space.is_pathable());
        let space = Space::Empty;
        assert!(space.is_pathable());
    }
}
