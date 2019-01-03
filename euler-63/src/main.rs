fn main() {
    let mut counter = 0;
    for start in 9..10 {
        let mut power = 1;
        while number_digits(npow(start, power)) == power {
            println!("{}", npow(start, power));
            power += 1;
            counter += 1;
        }
    }

    println!("Number powerful digit counts {}", counter);
}

fn npow(base: i64, exponent: i64) -> i64 {
    let mut result = 1;
    for _ in 0..exponent {
        result *= base;
    }

    result
}

const DIGITS:[u64; 13] = [10, 100, 1000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000];

fn number_digits(number: i64) -> i64 {
    let n: u64 = if number < 0 {
        (-number) as u64
    } else {
        number as u64
    };

    let mut digits = 0;

    while digits < DIGITS.len() && DIGITS[digits] <= n {
        digits += 1;
    }

    if digits == DIGITS.len() {
        let mut dividend = n / DIGITS.last().unwrap();

        while dividend > 0 {
            dividend /= 10;
            digits += 1;
        }
        digits as i64
    } else {
        (digits + 1) as i64
    }
}
