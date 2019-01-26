use rug::Integer;

pub fn find_pell_eq_solution(d: Integer) -> (Integer, Integer) {
    let a0 = d.clone().sqrt();

    let mut P = a0.clone();
    let mut Q = d.clone() - a0.clone().square();
    let mut a = (a0.clone() + P.clone())/Q.clone();
    let mut p = a0.clone() * a.clone() + Integer::from(1);
    let mut p_old = a0.clone();
    let mut q = a.clone();
    let mut q_old = Integer::from(1);

    let mut r = 1;

    while 2 * a0.clone() != a {
        P = a.clone() * Q.clone() - P;
        Q = (d.clone() - P.clone().square())/Q.clone();
        a = (a0.clone() + P.clone())/Q.clone();
        let p_new = a.clone() * p.clone() + p_old;
        let q_new = a.clone() * q.clone() + q_old;

        q_old = q;
        q = q_new;

        p_old = p;
        p = p_new;

        r += 1;
    }

    if r % 2 == 0 {
        (p_old, q_old)
    } else {
        let mut r_new = 0;

        while r_new < r - 1 {
            P = a.clone() * Q.clone() - P;
            Q = (d.clone() - P.clone().square())/Q.clone();
            a = (a0.clone() + P.clone())/Q.clone();
            let p_new = a.clone() * p.clone() + p_old;
            let q_new = a.clone() * q.clone() + q_old;

            q_old = q;
            q = q_new;

            p_old = p;
            p = p_new;

            r_new += 1;
        }

        (p, q)
    }
}

fn is_sqrt(d: u64) -> bool {
    let sqrt = (d as f64).sqrt() as u64;

    sqrt * sqrt == d
}

pub fn find_largest_minimal_solution(max_d: u64) -> (Integer, u64) {
    (2..(max_d + 1))
        .filter(|&d| !is_sqrt(d))
        .map(|d| (find_pell_eq_solution(Integer::from(d)).0, d))
        .max_by_key(|t| t.clone().0)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_largest_minimal_solution() {
        assert_eq!(find_largest_minimal_solution(7), (Integer::from(9), 5));
    }

    #[test]
    fn test_find_pell_eq_solution_7() {
        assert_eq!(find_pell_eq_solution(Integer::from(7)), (Integer::from(8), Integer::from(3)));
    }

    #[test]
    fn test_find_pell_eq_solution_13() {
        assert_eq!(find_pell_eq_solution(Integer::from(13)), (Integer::from(649), Integer::from(180)));
    }
}
