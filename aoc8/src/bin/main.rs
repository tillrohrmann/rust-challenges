use aoc8::{read_input_from_file, TreeNode};

fn main() {
    let input = read_input_from_file("input.txt").unwrap();

    let tree = TreeNode::parse_tree(&input).unwrap();

    println!("{}", tree.sum_meta_data());
}