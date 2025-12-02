use crate::{solution::Solution, year2025::day1::dial::Dial};

pub fn solve(data: &str) -> Solution<u32, u32> {
    let mut dial = Dial::default();
    let mut rotation;

    for line in data.lines() {
        rotation = line.parse().unwrap();
        dial.rotate(rotation);
    }

    Solution::new(dial.visited_zeros_strict(), dial.visited_zeros_loose())
}
