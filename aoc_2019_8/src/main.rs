fn main() {
    let image = aoc_2019_8::Image::load_from_file(25, 6, "input.txt").unwrap();

    let check_sum = image.check_sum().unwrap();

    println!("Checksum {}", check_sum);

    image.print_rendered_image();
}
