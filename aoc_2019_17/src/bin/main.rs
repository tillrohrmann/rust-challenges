fn main() {
    solve_part_two();
}

fn solve_part_two() {
    let mut program = aoc_2019_2::read_memory_from_file("input.txt");
    program[0] = 2;
    let mut vacuum_cleaner = aoc_2019_17::VacuumCleaner::new(&program);

    vacuum_cleaner.execute();
}

fn solve_part_one() {
    let program = aoc_2019_2::read_memory_from_file("input.txt");
    let scaffolding = aoc_2019_17::Scaffolding::new(&program);
    let map = scaffolding.extract_scaffolding().unwrap();
    let intersections = aoc_2019_17::find_intersections(&map);
    let result: isize = intersections
        .iter()
        .map(|&aoc_common::math::Point(x, y)| x * y)
        .sum();
    println!("{}", result);
}
