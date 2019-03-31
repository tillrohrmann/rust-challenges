use aoc14::Recipes;

fn main() {
//    solve_part_one();
    solve_part_two();
}

fn solve_part_one() {
    let mut recipes = Recipes::new();
    println!("{:?}", recipes.find_recipes_after(236021, 10));
}

fn solve_part_two() {
    let mut recipes = Recipes::new();
    println!("{}", recipes.number_recipes_until_sequence(&[2,3,6,0,2,1]));
}