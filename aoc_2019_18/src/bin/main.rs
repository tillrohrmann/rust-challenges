use aoc_2019_18::solve_map;

fn main_other() {
    let map = aoc_2019_18::read_map("input.txt").unwrap();

    let result = solve_map(&map);
    println!("{}", result)
}