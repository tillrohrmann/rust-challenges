use aoc14::Recipes;

fn main() {
    let mut recipes = Recipes::new();

    println!("{:?}", recipes.find_recipes_after(236021, 10));
}