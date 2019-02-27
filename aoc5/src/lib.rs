pub fn react_polymer(input: &str) -> String {
    let mut stack: Vec<char> = Vec::with_capacity(input.len());

    for chr in input.chars() {
        if let Some(&last) = stack.last() {
            if is_opposite(chr, last) {
                stack.pop();
            } else {
                stack.push(chr);
            }
        } else {
            stack.push(chr);
        }
    }

    stack.into_iter().collect()
}

fn is_opposite(a: char, b: char) -> bool {
    a != b && (a.to_ascii_lowercase() == b || a == b.to_ascii_lowercase())
}

pub fn is_minimal(input: &str) -> bool {
    let characters: Vec<char> = input.chars().collect();
    for i in 0..(input.len() -1) {
        if is_opposite(characters[i], characters[i + 1]) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react_polymer() {
        assert_eq!(react_polymer("dabAcCaCBAcCcaDA"), "dabCBAcaDA")
    }

    fn test_is_opposite_1() {
        assert_eq!(is_opposite('a', 'a'), false)
    }

    fn test_is_opposite_2() {
        assert_eq!(is_opposite('A', 'a'), true)
    }

    fn test_is_opposite_3() {
        assert_eq!(is_opposite('A', 'A'), false)
    }

    fn test_is_opposite_4() {
        assert_eq!(is_opposite('a', 'A'), true)
    }
}
