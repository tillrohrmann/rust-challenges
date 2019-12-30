use std::collections::HashMap;
use std::fs;

struct OrbitCounter {
    orbits: HashMap<String, Vec<String>>,
}

impl OrbitCounter {
    fn parse_from_string(input: &str) -> Result<OrbitCounter, String> {
        let orbits = OrbitCounter::parse_orbits_from_string(input)?;

        Ok(OrbitCounter { orbits })
    }

    fn parse_orbits_from_string(input: &str) -> Result<HashMap<String, Vec<String>>, String> {
        let relations: Result<Vec<(String, String)>, String> = input
            .split("\n")
            .map(|word| {
                let mut splits = word.trim().split(")");

                let parent = splits.next();
                let child = splits.next();

                match (parent, child) {
                    (Some(parent), Some(child)) => Ok((parent.to_string(), child.to_string())),
                    _ => Err("Could not extract parent children relation from input.".to_string()),
                }
            })
            .collect();

        relations.map(|relations| {
            let mut result = HashMap::new();

            for (parent, child) in relations {
                result.entry(parent).or_insert_with(Vec::new).push(child);
            }

            result
        })
    }

    fn count_orbits(&self) -> u32 {
        self.count_orbits_from("COM").0
    }

    fn count_orbits_from(&self, start: &str) -> (u32, u32) {
        match self.orbits.get(start) {
            None => (0, 1),
            Some(children) => {
                let (paths, nodes) = children
                    .iter()
                    .map(|child| self.count_orbits_from(child))
                    .fold((0, 0), |(acc_paths, acc_nodes), (paths, nodes)| {
                        (acc_paths + paths, acc_nodes + nodes)
                    });

                (paths + nodes, nodes + 1)
            }
        }
    }
}

pub fn calculate_orbits_from_file(filename: &str) -> Result<u32, String> {
    let input =
        fs::read_to_string(filename).map_err(|err| format!("Could not read file: {}", err))?;

    let orbit_counter = OrbitCounter::parse_from_string(&input.trim())?;

    Ok(orbit_counter.count_orbits())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = "\
COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

        let orbit_counter = OrbitCounter::parse_from_string(input).unwrap();

        assert_eq!(orbit_counter.count_orbits(), 42)
    }

    #[test]
    fn exmple_two() {
        let input = "\
COM)J
J)K
K)L
COM)F";

        let orbit_counter = OrbitCounter::parse_from_string(input).unwrap();

        assert_eq!(orbit_counter.count_orbits(), 7)
    }
}
