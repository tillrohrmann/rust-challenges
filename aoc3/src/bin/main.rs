use aoc3::*;

fn main() {
    let proposals: Vec<Proposal> = read_file("input.txt").iter().map(|line| parse_proposal(line).unwrap()).collect();

    let mut fabric = Fabric::new();

    for x in proposals {
        fabric.add(&x);
    }

    println!("{}", fabric.count());
}