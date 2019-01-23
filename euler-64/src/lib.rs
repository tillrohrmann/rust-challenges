use std::slice::Iter;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Fraction(Times, Const);

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Times(Const, Addition);

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Addition(Sqrt, Const);

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Sqrt{v: u32, floor: u32}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
struct Const(i32);

impl Sqrt {
    fn new(v: u32) -> Sqrt {
        let floor= (v as f64).sqrt().floor() as u32;
        Sqrt { v, floor}
    }
}

fn one() -> Const {
    Const(1)
}

fn one_addition() -> Addition {
    Addition(Sqrt::new(1), Const(0))
}

fn sqrt_addition(v: u32) -> Addition {
    Addition(Sqrt::new(v), Const(0))
}

fn sqrt_times(v: u32) -> Times {
    Times(Const(1), sqrt_addition(v))
}

pub fn sqrt(v: u32) -> Fraction {
    Fraction(sqrt_times(v), one())
}

pub fn fraction(a: i32, b: i32) -> Fraction {
    Fraction(Times(Const(a), one_addition()), Const(b))
}

pub fn cancel(primes: &Primes, fraction: Fraction) -> Fraction {
    match fraction {
        Fraction(Times(Const(a), b), Const(c)) => {
            let gcd = primes.gcd(a, c);
            Fraction(Times(Const(a / gcd as i32), b), Const(c / gcd as i32))
        }
    }
}

pub fn next_fraction_pair(primes: &Primes, fraction: Fraction) -> (i32, Fraction) {
    let fraction = cancel(primes, fraction);

    match fraction {
        Fraction(Times(Const(a), Addition(s, Const(b))), Const(c)) => {
            let upper_bound = s.floor as i32 + b;
            let value = upper_bound / c;

            let new_b = b - value * c;

            (value, Fraction(Times(Const(a), Addition(s, Const(new_b))), Const(c)))
        }
    }
}

fn next_fraction(fraction: Fraction) -> Fraction {
    let Fraction(Times(Const(a), Addition(s, Const(b))), c) = fraction;

    let sv = s.v;
    Fraction(Times(c, Addition(s, Const(-b))), Const(a * (sv as i32 - b*b)))
}

pub struct SqrtSequence<'a> {
    fraction: Fraction,
    primes: &'a Primes,
}

impl SqrtSequence<'_> {
    pub fn new(v: u32, primes: &'_ Primes) -> SqrtSequence<'_> {
        SqrtSequence {
            fraction: sqrt(v),
            primes
        }
    }
}

impl Iterator for SqrtSequence<'_> {
    type Item = (i32, Fraction);

    fn next(&mut self) -> Option<Self::Item> {

        if not_zero(self.fraction) {
            let (value, remainder) = next_fraction_pair(self.primes, self.fraction);
            let next_fraction = cancel(self.primes, next_fraction(remainder));

            self.fraction = next_fraction;

            Some((value, self.fraction))
        } else {
            None
        }
    }
}

pub struct Primes {
    primes: Vec<usize>
}

impl Primes {
    pub fn new(number_primes: usize) -> Primes {
        let mut sieve = vec![true; number_primes];

        sieve[0] = false;
        sieve[1] = false;

        let mut counter = 0;

        for i in 2..(number_primes as f64).sqrt().ceil() as usize {
            if sieve[i] {
                counter += 1;
                for j in ((2 * i)..number_primes).step_by(i) {
                    sieve[j] = false;
                }
            }
        }

        let mut primes = Vec::with_capacity(counter);

        for i in 0..number_primes {
            if sieve[i] {
                primes.push(i);
            }
        }
        Primes{primes}
    }

    fn iter(&self) -> Iter<usize> {
        self.primes.iter()
    }

    fn gcd(&self, a: i32, b: i32) -> u32 {
        let mut a = a.abs() as u32;
        let mut b = b.abs() as u32;
        let mut gcd = 1;
        let mut prime_iterator = self.iter();

        while a > 1 && b > 1 {
            let prime = *prime_iterator.next().unwrap() as u32;

            let mut counter_a = 0;

            while a % prime == 0 {
                a /= prime;
                counter_a += 1;
            }

            let mut counter_b = 0;

            while b % prime == 0 {
                b /= prime;
                counter_b += 1;
            }

            for _ in 0..u32::min(counter_a, counter_b) {
                gcd *= prime;
            }
        }

        gcd
    }
}

fn not_zero(fraction: Fraction) -> bool {
    let Fraction(_, Const(v)) = fraction;

    v != 0
}

pub fn calculate_sequence_length(v: u32, primes: &Primes) -> usize {
    let mut fractions = std::collections::HashSet::new();

    let mut sqrt_seq = SqrtSequence::new(v, primes);

    sqrt_seq.take_while(|(_, fraction)| not_zero(*fraction) && fractions.insert(*fraction)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sequence_length9() {
        let primes = Primes::new(100);
        assert_eq!(calculate_sequence_length(9, &primes), 0)
    }

    #[test]
    fn test_calculate_sequence_length23() {
        let primes = Primes::new(100);
        assert_eq!(calculate_sequence_length(23, &primes), 4)
    }

    #[test]
    fn test_calculate_sequence_length7() {
        let primes = Primes::new(100);
        assert_eq!(calculate_sequence_length(7, &primes), 4)
    }

    fn test_sqrt_sequence(v: u32, t: usize) -> Vec<i32> {
        let primes = Primes::new(100);
        let sqrt_seq = SqrtSequence::new(v, &primes);

        sqrt_seq.map(|(v, _)| v).take(t).collect()
    }

    #[test]
    fn test_sqrt_sequence23() {
        assert_eq!(test_sqrt_sequence(23, 5), vec![4,1,3,1,8]);
    }

    #[test]
    fn test_sqrt_sequence2() {
        assert_eq!(test_sqrt_sequence(2, 3), vec![1, 2, 2])
    }

    #[test]
    fn test_sqrt_sequence3() {
        assert_eq!(test_sqrt_sequence(3, 3), vec![1, 1, 2])
    }

    #[test]
    fn test_sqrt_sequence13() {
        assert_eq!(test_sqrt_sequence(13, 7), vec![3, 1, 1, 1, 1, 6, 1])
    }

    #[test]
    fn test_next_fraction() {
        let fraction = Fraction(Times(Const(1), Addition(Sqrt::new(23), Const(-3))), Const(7));

        assert_eq!(next_fraction(fraction), Fraction(Times(Const(7), Addition(Sqrt::new(23), Const(3))), Const(14)));
    }

    #[test]
    fn test_next_number() {
        let primes = Primes::new(100);
        let fraction = Fraction(Times(Const(1), Addition(Sqrt::new(23), Const(4))), Const(7));

        let (value, new_fraction) = next_fraction_pair(&primes, fraction);

        assert_eq!(value, 1);
        assert_eq!(new_fraction, Fraction(Times(Const(1), Addition(Sqrt::new(23), Const(-3))), Const(7)));
    }

    #[test]
    fn test_primes() {
        let primes = Primes::new(10);

        let result:Vec<usize> = primes.primes.into_iter().collect();

        assert_eq!(result, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_gcd_aa() {
        let primes = Primes::new(10);
        assert_eq!(primes.gcd(1, 1), 1)
    }

    #[test]
    fn test_gcd_ab() {
        let primes = Primes::new(10);
        assert_eq!(primes.gcd(4, 2), 2)
    }

    #[test]
    fn test_gcd_ba() {
        let primes = Primes::new(10);
        assert_eq!(primes.gcd(2, 4), 2)
    }

    #[test]
    fn test_gcd_ac() {
        let primes = Primes::new(10);
        assert_eq!(primes.gcd(7, 5), 1)
    }
}