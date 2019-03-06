use aoc7::{DependencyGraph, WorkAssignment, Time};

fn main() {
    let dependencies = aoc7::read_dependencies("input.txt").unwrap();

    println!("Dependencies: {:?}", dependencies);

    let graph = DependencyGraph::generate_graph(&dependencies);

    let topological_order = graph.sort_topologically();

    println!("Topological sort order: {}", topological_order.iter().collect::<String>());

    let required_time = graph.assign_work(5, 60).last().map(|&WorkAssignment(_, _, Time(_, end_time))| end_time).unwrap_or(0);

    println!("Required time: {}", required_time);
}