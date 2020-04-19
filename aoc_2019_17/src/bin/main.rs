fn main() {
    let program = aoc_2019_2::read_memory_from_file("input.txt");
    let scaffolding = aoc_2019_17::Scaffolding::new(&program);
    let map = scaffolding.extract_scaffolding().unwrap();
    let intersections = aoc_2019_17::find_intersections(&map);

    let result: isize = intersections.iter().map(|&aoc_common::math::Point(x, y)| x * y).sum();

    println!("{}", result);
}
