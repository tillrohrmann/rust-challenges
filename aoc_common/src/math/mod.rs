use std::fmt::Display;
use std::ops::{Add, Sub};

use crate::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn length(&self) -> f64 {
        let Point(x, y) = self;

        ((x * x + y * y) as f64).sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        let Point(x, y) = self;
        let Point(r_x, r_y) = rhs;

        Point(x + r_x, y + r_y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        let Point(x, y) = self;
        let Point(r_x, r_y) = rhs;

        Point(x - r_x, y - r_y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Point({},{})", self.0, self.1))
    }
}
