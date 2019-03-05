use aoc7::DependencyGraph;

fn main() {
    let dependencies = aoc7::read_dependencies("input.txt").unwrap();

    println!("Dependencies: {:?}", dependencies);

    let graph = DependencyGraph::generate_graph(&dependencies);

    println!("Graph: {:?}", graph);
}