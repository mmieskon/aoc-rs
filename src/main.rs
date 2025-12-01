use aoc_rs::year2025::day1;

fn main() {
    let path: &str = &std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(path).unwrap();

    let (passwd_part1, passwd_part2) = day1::calculate_password(&data);
    println!("Day1");
    println!("  - Part1: {passwd_part1}");
    println!("  - Part2: {passwd_part2}");
}
