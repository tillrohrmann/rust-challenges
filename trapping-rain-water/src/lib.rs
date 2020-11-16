// https://leetcode.com/problems/trapping-rain-water/

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let maximum_height = match height.iter().max() {
            Some(&height) => height,
            None => 0
        };

        let mut sum_trap = 0;

        let mut vertical = vec![i32::max_value(); maximum_height as usize];

        for (x, &height) in height.iter().enumerate() {
            for i in 0..(height as usize) {
                if vertical[i] < x as i32 {
                    sum_trap += x as i32 - vertical[i];
                }

                vertical[i] = x as i32 + 1;
            }
        }

        sum_trap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }
}
