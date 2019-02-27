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

pub fn find_minimal_polymer(input: &str) -> (char, usize) {
    (b'a'..=b'z').map(|v| {
        let chr = char::from(v);
        let modified_input = remove_char(input, chr);
        (chr, react_polymer(&modified_input).len())
    }).min_by(|(_, a_len), (_, b_len)| a_len.cmp(b_len)).unwrap()
}

fn remove_char(input: &str, chr: char) -> String {
    input.chars().filter(|&a| a.to_ascii_lowercase() != chr).collect()
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

    #[test]
    fn test_find_min_polymer() {
        assert_eq!(find_minimal_polymer("dabAcCaCBAcCcaDA"), ('c', 4))
    }

    #[test]
    fn test_is_opposite_1() {
        assert_eq!(is_opposite('a', 'a'), false)
    }

    #[test]
    fn test_is_opposite_2() {
        assert_eq!(is_opposite('A', 'a'), true)
    }

    #[test]
    fn test_is_opposite_3() {
        assert_eq!(is_opposite('A', 'A'), false)
    }

    #[test]
    fn test_is_opposite_4() {
        assert_eq!(is_opposite('a', 'A'), true)
    }
}
