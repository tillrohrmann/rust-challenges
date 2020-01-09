use std::io;
use std::fs;
use std::io::BufRead;
use std::error;
use std::fmt;

pub mod math;

pub type GenericResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct GenericError {
    message: String
}

impl GenericError {
    pub fn new(message: &str) -> GenericError {
        GenericError {
            message: message.to_string()
        }
    }
}

impl error::Error for GenericError {}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Generic error: {}", self.message)
    }
}

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
