use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use aoc_common::GenericResult;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum MapEntry {
    Start,
    End,
    SmallCave(String),
    BigCave(String),
}

impl MapEntry {
    fn is_small_cave(&self) -> bool {
        match self {
            MapEntry::Start => false,
            MapEntry::End => false,
            MapEntry::SmallCave(_) => true,
            MapEntry::BigCave(_) => false,
        }
    }
}

impl FromStr for MapEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "start" => Ok(MapEntry::Start),
            "end" => Ok(MapEntry::End),
            other => {
                let is_big_cave = other.chars().all(|chr| chr.is_ascii_uppercase());
                let is_small_cave = other.chars().all(|chr| chr.is_ascii_lowercase());

                if is_big_cave {
                    Ok(MapEntry::BigCave(other.to_string()))
                } else if is_small_cave {
                    Ok(MapEntry::SmallCave(other.to_string()))
                } else {
                    Err(format!("Invalid format {}.", other))
                }
            }
        }
    }
}

pub struct Map {
    neighbours: HashMap<MapEntry, Vec<MapEntry>>,
}


impl Map {
    fn new(neighbours: HashMap<MapEntry, Vec<MapEntry>>) -> Map {
        Map {
            neighbours
        }
    }

    pub fn parse(content: &Vec<String>) -> GenericResult<Map> {
        let mut neighbours = HashMap::new();

        for line in content {
            let mut splits = line.split("-");
            let left: MapEntry = splits.next().ok_or("Could not find left element.")?.parse()?;
            let right: MapEntry = splits.next().ok_or("Could not find right element.")?.parse()?;

            neighbours.entry(left.clone()).or_insert(Vec::new()).push(right.clone());
            neighbours.entry(right).or_insert(Vec::new()).push(left)
        }

        Ok(Map::new(neighbours))
    }

    pub fn count_distinct_paths(&self) -> usize {
        let mut states = vec![State::start()];
        let mut distinct_paths = 0;

        while let Some(state) = states.pop() {
            if state.entry == MapEntry::End {
                distinct_paths += 1;
            } else {
                if let Some(next_entries) = self.neighbours.get(&state.entry) {
                    for next_entry in next_entries {
                        if let Some(next_state) = state.go(next_entry) {
                            states.push(next_state);
                        }
                    }
                }
            }
        }

        distinct_paths
    }
}

#[derive(Debug)]
struct State {
    entry: MapEntry,
    visited_entries: HashSet<MapEntry>,
}

impl State {
    fn start() -> State {
        let mut visited_entries = HashSet::new();
        visited_entries.insert(MapEntry::Start);
        State {
            entry: MapEntry::Start,
            visited_entries,
        }
    }

    fn new(entry: MapEntry, visited_entries: HashSet<MapEntry>) -> State {
        State {
            entry,
            visited_entries,
        }
    }

    fn go(&self, next_entry: &MapEntry) -> Option<State> {
        if next_entry.is_small_cave() && self.visited_entries.contains(next_entry) || *next_entry == MapEntry::Start {
            None
        } else {
            let mut new_visited_entries = self.visited_entries.clone();
            new_visited_entries.insert(next_entry.clone());
            Some(State::new(next_entry.clone(), new_visited_entries))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end".split("\n").map(|line| line.to_string()).collect();

        let map = Map::parse(&input).unwrap();
        assert_eq!(map.count_distinct_paths(), 10);
    }
}
