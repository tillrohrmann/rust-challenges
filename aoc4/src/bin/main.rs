fn main() {
    let log = aoc4::read_log("input.txt").unwrap();

    let guard_overview = aoc4::process_log(&log);

    let max_guard = guard_overview.iter().max_by(|&a, &b| a.get_total_minutes_asleep().cmp(&b.get_total_minutes_asleep())).unwrap();

    println!("{:?}", max_guard);

    println!("{}", max_guard.get_id() as usize * max_guard.find_minute_most_often_asleep());
}