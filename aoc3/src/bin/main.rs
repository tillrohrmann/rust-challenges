use aoc3::*;

fn main() {
    let proposals: Vec<Proposal> = read_file("input.txt").iter().map(|line| parse_proposal(line).unwrap()).collect();

    let mut fabric = Fabric::new();

    for x in proposals.iter() {
        fabric.add(x);
    }

    println!("{}", fabric.count());

    let result: Vec<Proposal> = proposals.into_iter().filter(|proposal| fabric.check(proposal)).collect();

    println!("{:?}", result)
}