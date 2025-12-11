use super::beam_simulator::BeamSimulatorClassical;
use super::beam_simulator2::BeamSimulatorQuantum;
use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<u32, u64> {
    let mut simulator_classical: BeamSimulatorClassical = data.parse().unwrap();
    let mut simulator_quantum: BeamSimulatorQuantum = data.parse().unwrap();

    simulator_classical.simulate();
    simulator_quantum.simulate();

    Solution {
        part1: simulator_classical.visited_splitters_count(),
        part2: simulator_quantum.visited_splitters_count(),
    }
}
