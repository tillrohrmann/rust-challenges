#[macro_use]
extern crate lazy_static;

use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct Rectangular {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rectangular {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangular {
        Rectangular {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Proposal {
    id: u32,
    rectangular: Rectangular,
}

impl Proposal {
    fn new(id: u32, rectangular: Rectangular) -> Proposal {
        Proposal {
            id,
            rectangular,
        }
    }
}

pub fn read_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(&file);

    reader.lines().map(|result| result.unwrap()).collect()
}

pub fn parse_proposal(line: &str) -> Result<Proposal, String> {
    lazy_static! {
        static ref PROPOSAL_RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }

    let regex: &Regex = &PROPOSAL_RE;

    if let Some(cap) = regex.captures(line) {
        let id = u32::from_str(&cap[1]).unwrap();
        let x = u32::from_str(&cap[2]).unwrap();
        let y = u32::from_str(&cap[3]).unwrap();
        let width = u32::from_str(&cap[4]).unwrap();
        let height = u32::from_str(&cap[5]).unwrap();

        let rect = Rectangular::new(x, y, width, height);

        Ok(Proposal::new(id, rect))
    } else {
        Err(format!("Could not parse input line: {}", line))
    }
}

pub struct Fabric {
    map: [[u32; 1000]; 1000]
}

impl Fabric {
    pub fn new() -> Fabric {
        Fabric {
            map: [[0; 1000]; 1000],
        }
    }

    pub fn add(&mut self, proposal: &Proposal) {
        let mut y = proposal.rectangular.y as usize;

        for _ in 0..proposal.rectangular.height {
            let mut x = proposal.rectangular.x as usize;

            for _ in 0..proposal.rectangular.width {
                self.map[y][x] += 1;
                x += 1;
            }

            y += 1;
        }
    }

    pub fn print(&self) {
        for y in 0..1000 {
            for x in 0..1000 {
                print!("{};", self.map[y][x])
            }
            print!("\n");
        }
    }

    pub fn count(&self) -> u32 {
        let mut counter = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                if self.map[y][x] > 1 {
                    counter += 1;
                }
            }
        }
        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_proposal() {
        assert_eq!(parse_proposal("#24 @ 61,509: 10x17").unwrap(), Proposal::new(24, Rectangular::new(61, 509, 10, 17)))
    }

    #[test]
    #[should_panic]
    fn test_failing_parse_proposal() {
        parse_proposal("foobar").unwrap();
    }
}
