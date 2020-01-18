use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

use crate::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn length(&self) -> f64 {
        let Point(x, y) = self;

        ((x * x + y * y) as f64).sqrt()
    }

    pub fn length_sqrd(&self) -> isize {
        let Point(x, y) = self;

        x * x + y * y
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

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, PartialOrd)]
pub struct Point3d(pub isize, pub isize, pub isize);

impl Add for Point3d {
    type Output = Point3d;

    fn add(self, rhs: Point3d) -> Self::Output {
        let Point3d(x, y, z) = self;
        let Point3d(r_x, r_y, r_z) = rhs;

        Point3d(x + r_x, y + r_y, z + r_z)
    }
}

impl Sub for Point3d {
    type Output = Point3d;

    fn sub(self, rhs: Point3d) -> Self::Output {
        let Point3d(x, y, z) = self;
        let Point3d(r_x, r_y, r_z) = rhs;

        Point3d(x - r_x, y - r_y, z - r_z)
    }
}

impl Mul<isize> for Point3d {
    type Output = Point3d;

    fn mul(self, rhs: isize) -> Self::Output {
        let Point3d(x, y, z) = self;

        Point3d(x * rhs, y * rhs, z * rhs)
    }
}
