use super::{part1::solve_part1, part2::solve_part2};
use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, usize> {
    Solution {
        part1: solve_part1(data),
        part2: solve_part2(data),
    }
}
