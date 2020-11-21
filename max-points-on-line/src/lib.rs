#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point {
            x,
            y
        }
    }
}

struct Solution {}

impl Solution {
    pub fn max_points(input: Vec<Vec<i32>>) -> i32 {
        if input.is_empty() {
            0
        } else {
            let points: Vec<Point> = input.iter().map(|vector| Point::new(vector[0] as isize, vector[1] as isize)).collect();
            let mut num_max_points_on_line = 1;

            for i in 0..input.len() {
                let a = points[i];
                for j in (i + 1)..input.len() {
                    let b = points[j];

                    if b != a {
                        let diff = Point::new(b.x - a.x, b.y - a.y);

                        let mut num_points_on_line = 0;
                        for k in 0..input.len() {
                            let point = points[k];
                            let other_diff = Point::new(point.x - a.x, point.y - a.y);

                            if diff.x * other_diff.y - diff.y * other_diff.x == 0 {
                                num_points_on_line += 1;
                            }
                        }

                        if num_points_on_line > num_max_points_on_line {
                            num_max_points_on_line = num_points_on_line;
                            println!("New max points {} start {:?}, end {:?}", num_max_points_on_line, a, b);
                        }
                    }
                }
            }

            if num_max_points_on_line == 1 {
                input.len() as i32
            } else {
                num_max_points_on_line
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        assert_eq!(Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
    }

    #[test]
    fn simple_case_two() {
        assert_eq!(Solution::max_points(vec![vec![1, 1], vec![3, 2], vec![5, 3], vec![4, 1], vec![2, 3], vec![1, 4]]), 4);
    }

    // [[0,0],[1,65536],[65536,0]]
    #[test]
    fn simple_case_three() {
        assert_eq!(Solution::max_points(vec![vec![0, 0], vec![1, 65536], vec![65536, 0]]), 2);
    }

    //[[1,1],[1,1],[0,0],[3,4],[4,5],[5,6],[7,8],[8,9]]
    #[test]
    fn simple_case_four() {
        assert_eq!(Solution::max_points(vec![vec![1, 1], vec![1, 1], vec![0, 0], vec![3, 4], vec![4,5], vec![5, 6], vec![7, 8], vec![8, 9]]), 5);
    }
}
