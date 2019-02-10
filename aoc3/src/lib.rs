#[macro_use]
extern crate lazy_static;

use std::fs;
use regex::Regex;
use std::str::FromStr;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    fn size(&self) -> usize {
        (self.width * self.height) as usize
    }
}

#[derive(Debug, PartialEq)]
pub struct Proposal {
    id: u32,
    rectangle: Rectangle,
}

impl Proposal {
    fn new(id: u32, rectangular: Rectangle) -> Proposal {
        Proposal {
            id,
            rectangle: rectangular,
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

        let rect = Rectangle::new(x, y, width, height);

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

    fn iter<'a>(&'a self, rectangle: &'a Rectangle) -> FabricIter<'a> {
        FabricIter::new(self, rectangle)
    }

    fn iter_mut<'a>(&'a mut self, rectangle: &'a Rectangle) -> FabricIterMut<'a> {
        FabricIterMut::new(self, rectangle)
    }

    pub fn add(&mut self, proposal: &Proposal) {
        self.iter_mut(&proposal.rectangle).for_each(|v| *v += 1);
    }

    pub fn print(&self) {
        let rectangle = Rectangle::new(0, 0, 1000, 1000);
        let a = self.iter(&rectangle);

        for (index, element) in a.enumerate() {
            print!("{};", element);

            if index % 1000 == 999 {
                print!("\n");
            }
        }
    }

    pub fn count(&self) -> usize {
        self.iter(&Rectangle::new(0, 0, 1000, 1000)).filter(|&v| v > 1).count()
    }

    pub fn check(&self, proposal: &Proposal) -> bool {
        self.iter(&proposal.rectangle).all(|v| v == 1)
    }
}

struct FabricIter<'a> {
    fabric : &'a Fabric,
    rectangle: &'a Rectangle,
    counter: usize,
}

impl<'a> FabricIter<'a> {
    fn new(fabric: &'a Fabric, rectangle: &'a Rectangle) -> FabricIter<'a> {
        FabricIter {
            fabric,
            rectangle,
            counter: 0
        }
    }
}

impl<'a> Iterator for FabricIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.rectangle.size() {
            None
        } else {
            let width = self.counter % self.rectangle.width as usize;
            let height = self.counter / self.rectangle.width as usize;

            self.counter += 1;

            let x = width + self.rectangle.x as usize;
            let y = height + self.rectangle.y as usize;

            Some(self.fabric.map[y][x])
        }
    }
}

struct FabricIterMut<'a> {
    fabric : &'a mut Fabric,
    rectangle: &'a Rectangle,
    counter: usize,
}

impl<'a> FabricIterMut<'a> {
    fn new(fabric: &'a mut Fabric, rectangle: &'a Rectangle) -> FabricIterMut<'a> {
        FabricIterMut {
            fabric,
            rectangle,
            counter : 0,
        }
    }
}

impl<'a> Iterator for FabricIterMut<'a> {
    type Item = &'a mut u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter >= self.rectangle.size() {
            None
        } else {
            let width = self.counter % self.rectangle.width as usize;
            let height = self.counter / self.rectangle.width as usize;

            self.counter += 1;

            let x = width + self.rectangle.x as usize;
            let y = height + self.rectangle.y as usize;

            let result = &mut self.fabric.map[y][x] as *mut u32;

            unsafe {
                Some(&mut *result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_proposal() {
        assert_eq!(parse_proposal("#24 @ 61,509: 10x17").unwrap(), Proposal::new(24, Rectangle::new(61, 509, 10, 17)))
    }

    #[test]
    #[should_panic]
    fn test_failing_parse_proposal() {
        parse_proposal("foobar").unwrap();
    }
}
