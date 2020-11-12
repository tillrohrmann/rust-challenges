fn main() {
    println!("foobar")
}

fn calculate_container(height: &Vec<i32>) -> i32 {
    let mut max_value = 0;
    for (index, &value) in height.iter().enumerate() {
        for (other_index, &other_value) in height.iter().enumerate().skip(index) {
            let value = (other_index as i32 - index as i32) * value.min(other_value);
            max_value = max_value.max(value);
        }
    }

    max_value
}

enum Foobar {
    A(i32),
    B{x: f32, y: bool},
}

#[cfg(test)]
mod tests {
    use crate::calculate_container;

    #[test]
    fn simple_test() {
        assert_eq!(calculate_container(&vec![1, 1]), 1)
    }

    #[test]
    fn simple_test_2() {
        assert_eq!(calculate_container(&vec![4, 3, 2, 1, 4]), 16)
    }

    #[test]
    fn simple_test_3() {
        assert_eq!(calculate_container(&vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
    }
}
