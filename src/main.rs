use aoc_rs::year2025::day1;

fn main() {
    let path: &str = &std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(path).unwrap();

    println!("{}", day1::solve(&data));
}
