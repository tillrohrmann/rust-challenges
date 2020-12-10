use std::collections::HashSet;

pub struct Group {
    members: Vec<String>,
}

impl Group {
    pub fn new(members: Vec<String>) -> Group {
        Group {
            members,
        }
    }

    pub fn count_unanimous_votes(&self) -> usize {
        let mut result = HashSet::new();

        for char in b'a'..=b'z' {
            result.insert(char as char);
        }

        for member in self.members.iter() {
            let votes: HashSet<char> = member.chars().collect();

            result.retain(|char| votes.contains(char))
        }

        result.len()
    }
}

pub fn create_groups(input: &Vec<String>) -> Vec<Group> {
    let mut result = Vec::new();
    let mut start = 0;

    for i in 0..input.len() {
        if input[i].is_empty() {
            let group = Group::new(input[start..i].to_vec());
            result.push(group);
            start = i + 1;
        }
    }

    if start < input.len() {
        let group = Group::new(input[start..input.len()].to_vec());
        result.push(group);
    }

    result
}

pub fn group_groups(input: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;

    for i in 0..input.len() {
        if input[i].is_empty() {
            result.push(input[start..i].join("").to_string());
            start = i + 1;
        }
    }

    if start < input.len() {
        result.push(input[start..input.len()].join("").to_string());
    }

    result
}

pub fn count_distinct_votes(input: &String) -> usize {
    let mut distinct_votes = HashSet::new();

    for char in input.chars() {
        distinct_votes.insert(char);
    }

    distinct_votes.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
