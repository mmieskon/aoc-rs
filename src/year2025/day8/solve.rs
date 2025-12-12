use std::{collections::HashSet, str::FromStr};

use crate::solution::Solution;

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

#[derive(Debug)]
struct JunctionBoxes {
    positions: Vec<Position>,
    visited_closest_pairs: HashSet<(usize, usize)>,
    circuits: Vec<HashSet<usize>>,
}

impl JunctionBoxes {
    fn solve(&mut self, iterations: usize) -> u32 {
        for _ in 0..iterations {
            let (idx1, idx2) = self.closest_boxes_next().unwrap();
            self.try_connect(idx1, idx2);
        }

        let mut ret: u32 = 1;
        self.circuits.sort_by_key(|set| set.len());

        for circuit in self.circuits.iter().rev().take(3) {
            ret *= u32::try_from(circuit.len()).unwrap();
        }

        ret
    }

    fn closest_boxes_next(&mut self) -> Option<(usize, usize)> {
        let mut dist_smallest: f64 = f64::INFINITY;
        let mut ret: Option<(usize, usize)> = None;

        for i in 0..self.positions.len() {
            for j in (i + 1)..self.positions.len() {
                let pos1 = &self.positions[i];
                let pos2 = &self.positions[j];
                let dist_curr = pos1.dist_relative(pos2);

                if (dist_curr < dist_smallest) && !self.visited_closest_pairs.contains(&(i, j)) {
                    dist_smallest = dist_curr;
                    ret = Some((i, j));
                }
            }
        }

        if let Some(pair) = ret {
            self.visited_closest_pairs.insert(pair);
        }

        ret
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
                    self.circuits[i].insert(idx1);
                }
                return;
            }
        }

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

        Ok(Self {
            positions,
            visited_closest_pairs: HashSet::new(),
            circuits: Vec::new(),
        })
    }
}

pub fn solve(data: &str) -> Solution<u32, &'static str> {
    let mut boxes: JunctionBoxes = data.parse().unwrap();
    let part1 = boxes.solve(1000);

    Solution {
        part1,
        part2: "TODO",
    }
}
