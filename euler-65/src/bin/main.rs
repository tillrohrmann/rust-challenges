fn main() {
    let fraction = euler_65::calculate_partial_euler_value(100);
    println!("{:?}", fraction);

    println!("{}", fraction.sum_numinator());
}
