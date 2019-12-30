struct PasswordVerifier {}

impl PasswordVerifier {
    fn is_valid_password(&self, password: u32, exact_double: bool) -> bool {
        let digits: Vec<u32> = PasswordVerifier::split_into_digits(password);

        PasswordVerifier::has_length(&digits, 6)
            && PasswordVerifier::has_double(&digits, exact_double)
            && PasswordVerifier::is_monotonic_increasing(&digits)
    }

    fn has_length(digits: &Vec<u32>, length: usize) -> bool {
        digits.len() == length
    }

    fn has_double(digits: &Vec<u32>, exact_double: bool) -> bool {
        let mut streak = 0;
        let mut last_digit = 10;

        for &digit in digits {
            if digit != last_digit {
                if (!exact_double || streak == 2) && streak >= 2 {
                    return true
                }

                streak = 0;
            }

            streak += 1;
            last_digit = digit;
        }

        (!exact_double || streak == 2) && streak >= 2
    }

    fn current_and_next_iterator(digits: &Vec<u32>) -> impl Iterator<Item = (&u32, &u32)>{
        digits.iter().zip(digits.iter().skip(1))
    }

    fn is_monotonic_increasing(digits: &Vec<u32>) -> bool {
        PasswordVerifier::current_and_next_iterator(digits).all(|(left, right)| left <= right)
    }

    fn split_into_digits(password: u32) -> Vec<u32> {
        let mut result = Vec::new();
        let mut password = password;

        while password > 0 {
            result.push(password % 10);
            password /= 10;
        }

        result.reverse();

        result
    }

    fn new() -> PasswordVerifier {
        PasswordVerifier {}
    }
}

pub fn valid_passwords_in_range(start: u32, end: u32, exact_double: bool) -> u32 {
    let mut valid_passwords = 0;
    let password_verifier = PasswordVerifier::new();

    for password in start..=end {
        if password_verifier.is_valid_password(password, exact_double) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn example_password_one() {
        let password_verifier = PasswordVerifier::new();

        assert!(password_verifier.is_valid_password(111111, false))
    }

    #[test]
    fn example_password_two() {
        let password_verifier = PasswordVerifier::new();

        assert!(!password_verifier.is_valid_password(223450, false))
    }

    #[test]
    fn example_password_three() {
        let password_verifier = PasswordVerifier::new();

        assert!(!password_verifier.is_valid_password(123789, false))
    }
}
