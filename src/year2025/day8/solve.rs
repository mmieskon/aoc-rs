use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;

const ITER_COUNT: usize = 1000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: u32,
    y: u32,
    z: u32,
}

impl Position {
    fn dist_relative(&self, other: &Position) -> f64 {
        let diff_x = f64::from(other.x) - f64::from(self.x);
        let diff_y = f64::from(other.y) - f64::from(self.y);
        let diff_z = f64::from(other.z) - f64::from(self.z);

        diff_x.powi(2) + diff_y.powi(2) + diff_z.powi(2)
    }
}

#[derive(Debug, Clone)]
struct JunctionBoxes {
    positions: Vec<Position>,
    pairs_sorted_by_dist: Vec<(usize, usize)>,
    circuits: Vec<HashSet<usize>>,
    lonely_circuit_count: usize,
}

impl JunctionBoxes {
    fn solve_part1(&mut self, iterations: usize) -> u32 {
        for i in 0..iterations {
            let (idx1, idx2) = self.pairs_sorted_by_dist[i];
            self.try_connect(idx1, idx2);
        }

        let mut ret: u32 = 1;
        self.circuits.sort_by_key(|set| set.len());

        for circuit in self.circuits.iter().rev().take(3) {
            ret *= u32::try_from(circuit.len()).unwrap();
        }

        ret
    }

    fn solve_part2(&mut self) -> u64 {
        let mut i = 0;
        let mut idx1_opt = None;
        let mut idx2_opt = None;

        while self.lonely_circuit_count > 0 {
            let (idx1, idx2) = self.pairs_sorted_by_dist[i];
            idx1_opt = Some(idx1);
            idx2_opt = Some(idx2);
            self.try_connect(idx1, idx2);
            i += 1;
        }

        u64::from(self.positions[idx1_opt.unwrap()].x)
            * u64::from(self.positions[idx2_opt.unwrap()].x)
    }

    fn try_connect(&mut self, idx1: usize, idx2: usize) {
        for (i, circuit) in self.circuits.iter().enumerate() {
            if circuit.contains(&idx1) && circuit.contains(&idx2) {
                return;
            } else if circuit.contains(&idx1) {
                if let Some((idx, _)) = self
                    .circuits
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .find(|(_, set)| set.contains(&idx2))
                {
                    let set = self.circuits.remove(idx);
                    self.circuits[i].extend(set);
                } else {
                    self.lonely_circuit_count -= 1;
                    self.circuits[i].insert(idx2);
                }
                return;
            } else if circuit.contains(&idx2) {
                if let Some((idx, _)) = self
                    .circuits
                    .iter()
                    .enumerate()
                    .skip(i + 1)
                    .find(|(_, set)| set.contains(&idx1))
                {
                    let set = self.circuits.remove(idx);
                    self.circuits[i].extend(set);
                } else {
                    self.lonely_circuit_count -= 1;
                    self.circuits[i].insert(idx1);
                }
                return;
            }
        }

        self.lonely_circuit_count -= 2;
        self.circuits.push(HashSet::from([idx1, idx2]));
    }
}

impl FromStr for JunctionBoxes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut positions: Vec<Position> = Vec::new();

        for line in s.lines() {
            let mut splitted = line.split(',');
            let x: u32 = splitted.next().unwrap().parse().unwrap();
            let y: u32 = splitted.next().unwrap().parse().unwrap();
            let z: u32 = splitted.next().unwrap().parse().unwrap();

            positions.push(Position { x, y, z });
        }

        let mut pairs = Vec::new();
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                pairs.push(((i, j), (&positions[i], &positions[j])));
            }
        }
        pairs.sort_by(|(_, pair1), (_, pair2)| {
            pair1
                .0
                .dist_relative(pair1.1)
                .total_cmp(&pair2.0.dist_relative(pair2.1))
        });

        let pairs_sorted_by_dist: Vec<(usize, usize)> = pairs.iter().map(|x| x.0).collect();

        let ret = Self {
            lonely_circuit_count: positions.len(),
            positions,
            pairs_sorted_by_dist,
            circuits: Vec::new(),
        };

        Ok(ret)
    }
}

pub fn solve(data: &str) -> Solution<u32, u64> {
    let mut boxes1: JunctionBoxes = data.parse().unwrap();
    let mut boxes2 = boxes1.clone();

    let part1 = boxes1.solve_part1(ITER_COUNT);
    let part2 = boxes2.solve_part2();

    Solution { part1, part2 }
}
