fn main() {
    let program = aoc_2019_2::read_memory_from_file("input.txt");
    let scaffolding = aoc_2019_17::Scaffolding::new(&program);
    let map = scaffolding.extract_scaffolding().unwrap();

    println!("{}", map);
}
