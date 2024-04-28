#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn index(&self, width: usize) -> usize {
        self.y * width + self.x
    }

    pub fn in_bound(&self, width: usize, height: usize) -> bool {
        self.x < width && self.y < height
    }
}

// Assignment4_Tests Point
#[cfg(test)]
mod point_tests {

    use super::*;
    use rand::*;
    #[test]
    fn test_manual_point_index() {
        let point = Point::new(5, 5);
        let width = 10;
        let expected = 55; // (5 * 10 + 5)
        let actual = point.index(width);
        assert_eq!(expected, actual)
    }

    // Overflow occurs if input is usize::Max but for usability of the Point struct It is very
    // unlikely that a x and y will get that big
    #[test]
    fn test_fuzzy_point_index() {
        let fuzzy_amount = 1000;
        (0..fuzzy_amount)
            .map(|_| {
                std::thread::spawn(|| {
                    let min = 0;
                    let max = 1000000000;
                    let mut rng = rand::thread_rng();
                    let random_y = rng.gen_range(min..max);
                    let random_x = rng.gen_range(min..max);
                    let random_w = rng.gen_range(min..max);
                    Point::new(random_x, random_y).index(random_w)
                })
                .join()
                .ok()
            })
            .for_each(|thread| assert_ne!(None, thread));
    }
}
