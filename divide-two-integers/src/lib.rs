struct Solution {}

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut result = 0;
        let negative = (dividend >= 0) ^ (divisor >= 0);

        let mut dividend = dividend;

        if dividend == i32::min_value() {
            if divisor == i32::min_value() {
                return 1;
            } else if divisor == -1 {
                return i32::max_value();
            } else if divisor == 1 {
              return i32::min_value();
            } else if negative {
                dividend += divisor;
                result = -1;
            } else {
                dividend -= divisor;
                result = 1;
            }
        } else if divisor == i32::min_value() {
            return 0;
        }


        let mut dividend = dividend.abs();
        let divisor = divisor.abs();

        while dividend >= divisor {
            dividend -= divisor;

            result += if negative {
                -1
            } else {
                1
            };
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(Solution::divide(7, -3), -2);
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(Solution::divide(i32::min_value(), 1), i32::min_value());
    }

    #[test]
    fn simple_test_3() {
        assert_eq!(Solution::divide(i32::min_value(), i32::min_value()), 1);
    }

    #[test]
    fn simple_test_4() {
        assert_eq!(Solution::divide(i32::min_value(), -1), i32::max_value());
    }

    #[test]
    fn simple_test_5() {
        assert_eq!(Solution::divide(i32::min_value(), i32::min_value() + 1), 1);
    }
}
