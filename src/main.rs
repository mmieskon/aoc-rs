use aoc_rs::{
    args::{self, Args},
    year2025::{day1, day2},
};

fn main() {
    let args = Args::parse();
    let data = std::fs::read_to_string(&args.path).unwrap();

    match args.day {
        1 => println!("{}", day1::solve(&data)),
        2 => println!("{}", day2::solve(&data)),
        3..=12 => todo!(),
        _ => args::print_usage_and_exit(),
    };
}
