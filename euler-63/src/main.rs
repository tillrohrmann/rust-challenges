fn main() {
    let mut counter = 0;
    for number in 1..10 {
        let bound = upper_bound(number);
        counter += bound.floor() as i32;
        println!("{}:{}", number, bound);
    }

    println!("Number powerful digit counts {}", counter);
}

fn upper_bound(number: i32) -> f64 {
    1.0 / (1.0 - (number as f64).log10())
}
