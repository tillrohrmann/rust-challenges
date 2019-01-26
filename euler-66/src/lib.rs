use std::collections::BTreeMap;
use rayon::prelude::*;
use std::cmp::Ordering;

type Int = rug::Integer;

pub struct Sqrts {
    sqrts: BTreeMap<Int, Int>,
    min: Int,
    max: Int,
}

impl Sqrts {
    pub fn filled(max: u32) -> Sqrts {
        let mut sqrts = BTreeMap::new();

        for i in 1..=max {
            let int = rug::Integer::from(i);
            let sq = int.clone().square();
            sqrts.insert(sq, int);
        }

        Sqrts {
            sqrts,
            max: rug::Integer::from(max),
            min: rug::Integer::from(1),
        }
    }

    pub fn is_sq(&self, sq: &Int) -> bool {
        self.check_sq(sq);

        self.sqrts.contains_key(&sq)
    }

    fn check_sq(&self, value: &Int) {
        if self.max.cmp(&value) == Ordering::Less || self.min.cmp(&value) == Ordering::Greater {
            panic!("Value {} is not in range {:?} and {:?}.", value, self.min, self.max);
        }
    }

    pub fn sqrt(&self, value: &Int) -> Option<&Int> {
        self.check_sq(value);

        self.sqrts.get(&value).map(|v| v)
    }

    pub fn insert(&mut self, v: Int, sqrt: Int) {
        if self.max < v{
            self.max = v.clone();
        }

        if self.min > v {
            self.min = v.clone()
        }

        self.sqrts.insert(v, sqrt);
    }

    pub fn prune(&mut self, min: Int) {
        if self.min < min {
            self.sqrts = self.sqrts.split_off(&min);
            self.min = min;
        }
    }
}

pub fn is_diophantine_solution(x: usize, y: usize, d: usize) -> bool {
    x * x - d * y * y == 1
}

pub fn find_minimal_x(d: u32) -> Option<Int> {
    let mut x = rug::Integer::from(2);

    println!("{}", d);
    loop {
        let sq = x.clone().square();
        let a = sq - 1u32;

        if a.mod_u(d) == 0u32 && is_sq(a.div_exact_u(d)){
            return Some(x);
        } else {
            x += 1;
        }
    }

    None
}

fn is_sq(value: Int) -> bool {
    let h = value.mod_u(16);

    if h > 9 {
        false
    } else if h != 2 && h != 3 && h != 5 && h != 6 && h != 7 && h != 8 {
        let sqrt = value.clone().sqrt();

        sqrt.square() == value
    } else {
        false
    }

}

pub fn find_largest_minimal_x(max_d: u32) -> (Int, u32) {
    let mut largest_x = 0;
    let mut largest_d = 0;
    let sqrts = Sqrts::filled(max_d);

    (2u32..(max_d+1))
        .into_par_iter()
        .filter(|&d| !sqrts.is_sq(&rug::Integer::from(d)))
        .flat_map(|d| find_minimal_x(d).map(|v| (v, d)))
        .max_by_key(|v| v.0.clone())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_minimal_x() {
        assert_eq!(find_largest_minimal_x(7), (rug::Integer::from(9), 5));
    }

    #[test]
    fn test_find_minimal_x_2() {
        assert_eq!(find_minimal_x(2), Some(rug::Integer::from(3)));
    }

    #[test]
    fn test_find_minimal_x_13() {
        assert_eq!(find_minimal_x(13), Some(rug::Integer::from(649)));
    }

    #[test]
    fn test_find_minimal_x_5() {
        assert_eq!(find_minimal_x(5), Some(rug::Integer::from(9)));
    }

    #[test]
    fn test_find_minimal_x_7() {
        assert_eq!(find_minimal_x( 7), Some(rug::Integer::from(8)));
    }

    #[test]
    fn test_sqrts() {
        let sqrts = Sqrts::filled(100);
        assert!(sqrts.is_sq(&rug::Integer::from(9)));
        assert_eq!(sqrts.is_sq(&rug::Integer::from(8)), false);
        assert_eq!(sqrts.sqrt(&rug::Integer::from(81)), Some(&rug::Integer::from(9)));
    }
}
