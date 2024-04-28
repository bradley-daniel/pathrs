use crate::{point::Point, space::Space};

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
        if point.in_bound(self.width, self.height) {
            let index = point.index(self.width);
            self.spaces.get(index).copied()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, point: Point) -> Option<&mut Space> {
        if point.in_bound(self.width, self.height) {
            let index = point.index(self.width);
            self.spaces.get_mut(index)
        } else {
            None
        }
    }

    pub fn index(&self, point: Point) -> Option<usize> {
        if point.in_bound(self.width, self.height) {
            None
        } else {
            Some(self.unchecked_index(point))
        }
    }

    pub fn unchecked_index(&self, point: Point) -> usize {
        point.index(self.width)
    }

    pub fn clear(&mut self) -> &Self {
        for space in self.spaces.iter_mut() {
            *space = Space::Empty;
        }
        self
    }

    pub fn adjacent_points(&self, point: Point) -> Vec<Point> {
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

// Assignment4_Tests Grid
#[cfg(test)]
mod grid_tests {
    use super::{Grid, Space};
    use crate::point::Point;

    use rand::{thread_rng, Rng};
    use std::thread;

    #[test]
    fn manual_test_index() {
        let width = 5;
        let height = 5;
        let grid = Grid::new(width, height);
        let point = Point::new(2, 2);
        let expected = 12; // 2 * 5 + 2
        let actual = grid.unchecked_index(point);
        assert_eq!(expected, actual)
    }

    #[test]
    fn manual_test_clear() {
        let width = 5;
        let height = 5;
        let spaces: Vec<_> = (0..(width * height)).map(|_| Space::Obstacle).collect();
        let mut grid = Grid {
            spaces,
            width,
            height,
        };

        grid.clear();

        let expected = [Space::Empty].repeat(width * height);
        let actual = grid.spaces;
        assert_eq!(expected, actual)
    }

    #[test]
    fn manual_test_adjacent_points() {
        let width = 3;
        let height = 3;
        let spaces: Vec<_> = vec![
            Space::Empty,
            Space::Visited,
            Space::Empty,
            Space::Obstacle,
            Space::Empty,
            Space::Start(Point::new(3, 2)),
            Space::Empty,
            Space::Path,
            Space::Empty,
        ];
        let grid = Grid {
            spaces,
            width,
            height,
        };
        let actual = grid.adjacent_points(Point::new(1, 1));

        let expected = [
            Point::new(0, 1),
            Point::new(1, 2),
            Point::new(1, 0),
            Point::new(2, 1),
        ];

        assert_eq!(actual.len(), expected.len());
        let mut counter = 0;
        for value in expected.iter() {
            assert!(actual.contains(value));
            counter += 1;
        }
        assert!(expected.len() == counter)
    }

    // Found Cant have a grid size of width * height > usize::max
    #[test]
    fn fuzzy_test_get() {
        let fuzzy_test = 100;
        (0..fuzzy_test)
            .map(|_| {
                let mut rng = thread_rng();
                let width = rng.gen_range(0..100);
                let height = rng.gen_range(0..100);
                let grid = Grid::new(width, height);
                thread::spawn(move || {
                    let mut rng = thread_rng();
                    let x = rng.gen_range(0..usize::MAX);
                    let y = rng.gen_range(0..usize::MAX);
                    grid.get(Point::new(x, y));
                })
                .join()
                .ok()
            })
            .for_each(|result| {
                assert_ne!(None, result);
            })
    }
}
