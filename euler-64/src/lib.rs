use std::slice::Iter;

#[derive(Debug)]
pub struct Fraction(Times, Times);

#[derive(Debug)]
struct Times(Const, Addition);

#[derive(Debug)]
struct Addition(Sqrt, Const);

#[derive(Debug)]
struct Sqrt{v: u32, floor: u32}

#[derive(Debug)]
struct Const(u32);

impl Sqrt {
    fn new(v: u32) -> Sqrt {
        let floor= (v as f64).floor() as u32;
        Sqrt { v, floor}
    }
}

fn one() -> Times {
    Times(Const(1), Addition(Sqrt::new(1), Const(0)))
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

pub fn fraction(a: u32, b: u32) -> Fraction {
    Fraction(Times(Const(a), one_addition()), Times(Const(b), one_addition()))
}

pub fn cancel(primes: &Primes, fraction: Fraction) -> Fraction {
    match fraction {
        Fraction(Times(Const(a), b), Times(Const(c), d)) => {
            let gcd = primes.gcd(a, c);
            Fraction(Times(Const(a / gcd), b), Times(Const(c / gcd), d))
        }
        _ => panic!("Cancel problem")
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

        for i in 2..number_primes {
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

    fn gcd(&self, mut a: u32, mut b: u32) -> u32 {
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

#[cfg(test)]
mod tests {
    use super::*;

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