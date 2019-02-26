fn main() {
    let log = aoc4::read_log("input.txt");

    for logEntry in log.unwrap() {
        println!("{:?}", logEntry);
    }
}