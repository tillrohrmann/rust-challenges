use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

use crate::fmt;
use std::collections::HashMap;

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

#[derive(Debug)]
pub enum IntErrorKind {
    Overflow,
}

pub fn least_common_multiple<I: num_traits::PrimInt + std::hash::Hash>(
    numbers: &Vec<I>,
) -> Result<I, IntErrorKind> {
    let mut least_common_multiple_factors: HashMap<I, usize> = HashMap::new();

    for number in numbers {
        let factors = factorize(number.clone());

        for (factor, times) in factors {
            least_common_multiple_factors
                .entry(factor)
                .and_modify(|old| *old = std::cmp::max(*old, times))
                .or_insert(times);
        }
    }

    let mut result = I::one();

    for (factor, times) in least_common_multiple_factors {
        for _ in 0..times {
            result = result * factor;
        }
    }

    Ok(result)
}

pub fn factorize<I: num_traits::PrimInt + std::hash::Hash>(mut number: I) -> HashMap<I, usize> {
    let mut result: HashMap<I, usize> = HashMap::new();
    let mut current_factor = I::one() + I::one();

    while number > I::one() {
        while number.rem(current_factor) == I::zero() {
            number = number / current_factor;
            result
                .entry(current_factor)
                .and_modify(|t| *t += 1)
                .or_insert(1);
        }

        current_factor = current_factor + I::one();
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn trivial_least_common_multiple() {
        let value = 1;
        let result = least_common_multiple(&vec![value]).unwrap();

        assert_eq!(value, result);
    }

    #[test]
    fn simple_least_common_multiple() {
        let result = least_common_multiple(&vec![2, 3, 5]).unwrap();

        assert_eq!(2 * 3 * 5, result);
    }

    #[test]
    fn non_trivial_least_common_multiple() {
        let result = least_common_multiple(&vec![14, 6]).unwrap();

        assert_eq!(42, result);
    }
}
