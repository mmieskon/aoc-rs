use aoc_rs::{
    args::{self, Args},
    year2025::{day1, day2, day3, day4, day5, day6},
};

fn main() {
    let args = Args::parse();
    let data = std::fs::read_to_string(&args.path).unwrap();

    match args.day {
        1 => println!("Day1:\n{}", day1::solve(&data)),
        2 => println!("Day2:\n{}", day2::solve(&data)),
        3 => println!("Day3:\n{}", day3::solve(&data)),
        4 => println!("Day4:\n{}", day4::solve(&data)),
        5 => println!("Day5:\n{}", day5::solve(&data)),
        6 => println!("Day6:\n{}", day6::solve(&data)),
        7 => todo!(),
        8 => todo!(),
        9 => todo!(),
        10 => todo!(),
        11 => todo!(),
        12 => todo!(),
        _ => args::print_usage_and_exit(),
    };
}
