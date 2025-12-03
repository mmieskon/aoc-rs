use crate::{solution::Solution, year2025::day1::dial::Dial};

pub fn solve(data: &str) -> Solution<u32, u32> {
    let mut dial = Dial::default();
    let mut rotation;

    for line in data.lines() {
        rotation = line.parse().unwrap();
        dial.rotate(rotation);
    }

    Solution {
        part1: dial.visited_zeros_strict(),
        part2: dial.visited_zeros_loose(),
    }
}
