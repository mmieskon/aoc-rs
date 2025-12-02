use aoc_rs::{
    args::{self, Args},
    year2025::day1,
};

fn main() {
    let args = Args::parse();
    let data = std::fs::read_to_string(&args.path).unwrap();

    let solution = match args.day {
        1 => day1::solve(&data),
        2..=12 => todo!(),
        _ => args::print_usage_and_exit(),
    };

    println!("{solution}");
}
