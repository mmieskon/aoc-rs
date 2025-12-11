use super::beam_simulator::BeamSimulator;
use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<u32, &'static str> {
    let mut simulator: BeamSimulator = data.parse().unwrap();
    simulator.simulate();

    Solution {
        part1: simulator.visited_splitters_count(),
        part2: "TODO",
    }
}
