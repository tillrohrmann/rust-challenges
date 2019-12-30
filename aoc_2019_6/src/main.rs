fn main() {
    let orbit_counter = aoc_2019_6::create_orbit_counter_from_file("input.txt").unwrap();

    let number_orbits = orbit_counter.count_orbits();
    let orbit_distance = orbit_counter.distance_between("YOU", "SAN");

    println!("Number orbits: {}, distance between YOU and SAN: {}", number_orbits, orbit_distance);
}
