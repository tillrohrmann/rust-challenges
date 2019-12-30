use std::collections::HashMap;
use std::fs;

pub struct OrbitCounter {
    orbits: HashMap<String, Vec<String>>,
    parents: HashMap<String, String>,
}

impl OrbitCounter {
    fn parse_from_string(input: &str) -> Result<OrbitCounter, String> {
        let (orbits, parents) = OrbitCounter::parse_orbits_from_string(input)?;

        Ok(OrbitCounter { orbits, parents })
    }

    fn parse_orbits_from_string(input: &str) -> Result<(HashMap<String, Vec<String>>, HashMap<String, String>), String> {
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
            let mut parents = HashMap::new();
            let mut result = HashMap::new();

            for (parent, child) in relations {
                parents.insert(child.clone(), parent.clone());
                result.entry(parent).or_insert_with(Vec::new).push(child);
            }

            (result, parents)
        })
    }

    pub fn count_orbits(&self) -> u32 {
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

    pub fn distance_between(&self, src: &str, dst: &str) -> u32 {
        let mut path_to_root_from_src = self.path_to_root_from(src);
        let mut path_to_root_from_dst = self.path_to_root_from(dst);

        path_to_root_from_dst.reverse();
        path_to_root_from_src.reverse();

        let mut common_prefix_length = 0;

        while path_to_root_from_src[common_prefix_length] == path_to_root_from_dst[common_prefix_length] {
            common_prefix_length += 1;
        }

        (path_to_root_from_dst.len() as i32 + path_to_root_from_src.len() as i32 - 2 - 2 * common_prefix_length as i32) as u32
    }

    fn path_to_root_from(&self, src: &str) -> Vec<String> {
        let mut result = Vec::new();

        let mut src = src;

        result.push(src.to_string());

        while let Some(parent) = self.parents.get(src) {
            result.push(parent.clone());
            src = parent;
        }

        result
    }
}

pub fn calculate_orbits_from_file(filename: &str) -> Result<u32, String> {
    let orbit_counter = create_orbit_counter_from_file(filename)?;

    Ok(orbit_counter.count_orbits())
}

pub fn create_orbit_counter_from_file(filename: &str) -> Result<OrbitCounter, String> {
    let input =
        fs::read_to_string(filename).map_err(|err| format!("Could not read file: {}", err))?;
    let orbit_counter = OrbitCounter::parse_from_string(&input.trim())?;
    Ok(orbit_counter)
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
    fn example_two() {
        let input_two = "\
COM)J
J)K
K)L
COM)F";

        let orbit_counter = OrbitCounter::parse_from_string(input_two).unwrap();

        assert_eq!(orbit_counter.count_orbits(), 7)
    }

    #[test]
    fn example_distance() {
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
K)L
K)YOU
I)SAN";

        let orbit_counter = OrbitCounter::parse_from_string(input).unwrap();

        assert_eq!(orbit_counter.distance_between("YOU", "SAN"), 4)
    }
}
