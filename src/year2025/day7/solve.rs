use super::beam_simulator_classical::BeamSimulatorClassical;
use super::beam_simulator_quantum::BeamSimulatorQuantum;
use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<u32, u64> {
    let mut simulator_classical: BeamSimulatorClassical = data.parse().unwrap();
    let mut simulator_quantum: BeamSimulatorQuantum = data.parse().unwrap();

    simulator_classical.simulate();
    simulator_quantum.simulate();

    Solution {
        part1: simulator_classical.ans(),
        part2: simulator_quantum.ans(),
    }
}
