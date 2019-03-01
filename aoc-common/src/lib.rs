use std::io;
use std::fs;
use std::io::BufRead;

pub fn read_raw_file_content(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(&file);

    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_raw_file_content() {
        assert_eq!(read_raw_file_content("test_input.txt").unwrap(), vec!["foobar", "barfoo", "foo", "bar"]);
    }
}
