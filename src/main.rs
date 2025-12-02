use aoc_rs::{
    args::{self, Args},
    year2025::day1,
};

fn main() {
    let args = Args::parse();
    let data = std::fs::read_to_string(&args.path).unwrap();

    match args.day {
        1 => println!("{}", day1::solve(&data)),
        2..=12 => todo!(),
        _ => args::print_usage_and_exit(),
    }
}
