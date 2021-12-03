use std::collections::HashMap;

pub struct BagGraph {
    contains: HashMap<Bag, Vec<BagQuantity>>,
    is_contained: HashMap<Bag, Vec<Bag>>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Bag(String);

pub struct BagQuantity(Bag, usize);

impl BagGraph {
    pub fn new(contains: HashMap<Bag, Vec<BagQuantity>>, is_contained: HashMap<Bag, Vec<Bag>>) -> BagGraph {
        BagGraph{
            contains,
            is_contained,
        }
    }

    pub fn can_contain(&self, bag: &Bag) -> usize {
        0
    }
}

pub fn parse_bag_graph(input: &Vec<String>) -> BagGraph {
    for line in input {
        let mut splits = line.split("contain");

        splits.next()
    }
    BagGraph::new(HashMap::new(), HashMap::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
