use std::fs::File;
use std::io::{BufReader, BufRead};
use std::io::Error;
use std::io::Result;

pub fn find_maximum_path(filename: &str) -> Result<u64> {
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);

    let result: Vec<String> = buffered.lines().map(|line| line.unwrap()).collect();

    let rows: Vec<Vec<u64>> = result.iter().map(|line| line.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect()).rev().collect();

    let maximum_path = rows.into_iter().fold(Vec::new(), |acc, row| {
        if acc.is_empty() {
            row
        } else {
            assert!(acc.len() == row.len() + 1);

            row.iter().enumerate().map(|(index, value)| value + u64::max(acc[index], acc[index + 1])).collect()
        }
    });

    Ok(*maximum_path.get(0).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        assert_eq!(find_maximum_path("simple_triangle.txt").unwrap(), 23);
    }
}
