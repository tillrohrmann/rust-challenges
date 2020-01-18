use aoc_2019_11::{InputOutputComputer, IntOutputReader, PaintRobot, Color};
use aoc_2019_7::{AsyncIntComputer, ChannelReader, ChannelWriter};
use std::io::BufReader;
use std::sync::mpsc;

fn main() {
    solve_day_11_2();
}

fn solve_day_11_1() {
    let mut robot = create_robot();
    robot.paint(Color::Black);
    print_results(&robot);
}

fn print_results(robot: &PaintRobot) {
    println!(
        "At least once painted fields: {}",
        robot.get_num_at_least_once_painted_fields()
    );
    let painted_area = robot.painted_area_to_string();
    println!("{}", painted_area);
}

fn create_robot() -> PaintRobot {
    let memory: Vec<i64> = aoc_2019_2::read_memory_from_file("input.txt");
    let mut robot = PaintRobot::new(memory);
    robot
}

fn solve_day_11_2() {
    let mut robot = create_robot();
    robot.paint(Color::White);
    print_results(&robot);
}
