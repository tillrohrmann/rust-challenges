use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn find_two_numbers(input: &Vec<i32>) -> Result<i32, String> {
    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            if input[i] + input[j] == 2020 {
                return Ok(input[i] * input[j]);
            }
        }
    }

    Err("Could not find matching pair of numbers.".to_string())
}

pub fn read_numbers(path: &str) -> Result<Vec<i32>, String> {
    let lines = read_lines(path).map_err(|err| format!("Could not read file because {}", err))?;

    lines.iter().map(|line| line.parse().map_err(|err| format!("Could not parse integer because {}", err))).collect()
}

pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
