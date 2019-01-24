use std::ops::Add;

#[derive(Eq, PartialEq, Debug)]
pub struct Fraction(rug::Integer, rug::Integer);

impl Fraction {
    fn inverse(self) -> Fraction {
        let Fraction(num, denom) = self;
        Fraction(denom, num)
    }

    fn is_zero(&self) -> bool {
        self.0 == rug::Integer::from(0)
    }

    pub fn sum_numinator(&self) -> u64 {
        self.0.to_string().chars().map(|c| (c as u64) - ('0' as u64)).sum::<u64>()
    }
}

impl Add<rug::Integer> for Fraction {
    type Output = Fraction;

    fn add(self, rhs: rug::Integer) -> Self::Output {
        let Fraction(num, denom) = self;
        Fraction(rhs * &denom + num, denom)
    }
}

pub fn euler_sequence(index: usize) -> u64 {
    if index == 0 {
        2
    } else {
        let index = index - 1;

        if index % 3 == 0 || index % 3 == 2 {
            1
        } else {
            2 * (index / 3 + 1) as u64
        }
    }
}

pub fn calculate_partial_euler_value(number: usize) -> Fraction {
    (0..number).rev()
        .map(|i| euler_sequence(i))
        .fold(Fraction(rug::Integer::from(0), rug::Integer::from(1)), |fraction, v| {
            if fraction.is_zero() {
                Fraction(rug::Integer::from(v), rug::Integer::from(1))
            } else {
                fraction.inverse() + rug::Integer::from(v)
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euler_sequence() {
        assert_eq!((0..10).map(|i| euler_sequence(i)).collect::<Vec<u64>>(), vec![2,1,2,1,1,4,1,1,6,1])
    }

    #[test]
    fn test_calculate_partial_euler_value1() {
        assert_eq!(calculate_partial_euler_value(1), Fraction(rug::Integer::from(2), rug::Integer::from(1)))
    }

    #[test]
    fn test_calculate_partial_euler_value3() {
        assert_eq!(calculate_partial_euler_value(3), Fraction(rug::Integer::from(8), rug::Integer::from(3)))
    }

    #[test]
    fn test_calculate_partial_euler_value6() {
        assert_eq!(calculate_partial_euler_value(6), Fraction(rug::Integer::from(87), rug::Integer::from(32)))
    }

    #[test]
    fn test_calculate_partial_euler_value10() {
        assert_eq!(calculate_partial_euler_value(10), Fraction(rug::Integer::from(1457), rug::Integer::from(536)))
    }
}