use std::{collections::HashSet, ops::Index, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Item {
    Empty,
    Spawn,
    Splitter,
}

#[derive(Debug)]
pub struct BeamSimulatorClassical {
    map: Vec<Vec<Item>>,
    rows: usize,
    cols: usize,
    visited_positions: HashSet<Position>,
    ans: u32,
}

impl BeamSimulatorClassical {
    pub fn simulate(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let pos = Position { row, col };

                if matches!(self[&pos], Item::Spawn) {
                    self.simulate_beam(&pos);
                    return;
                }
            }
        }
        unreachable!();
    }

    pub fn ans(&self) -> u32 {
        self.ans
    }

    fn simulate_beam(&mut self, pos: &Position) {
        let mut pos = pos.clone();
        while self.in_bounds(&pos) {
            if !self.visited_positions.insert(pos.clone()) {
                return;
            }

            if matches!(self[&pos], Item::Splitter) {
                self.ans += 1;

                let mut pos_left = pos.clone();
                pos_left.col -= 1; // NOTE: Assumes this never goes below zero
                if self.in_bounds(&pos_left) {
                    self.simulate_beam(&pos_left);
                }

                let mut pos_right = pos.clone();
                pos_right.col += 1;
                if self.in_bounds(&pos_right) {
                    self.simulate_beam(&pos_right);
                }

                return;
            }

            pos.row += 1;
        }
    }

    fn in_bounds(&self, pos: &Position) -> bool {
        (pos.row < self.rows) && (pos.col < self.cols)
    }
}

impl FromStr for BeamSimulatorClassical {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<Item>> = Vec::new();

        for (i, line) in s.lines().enumerate() {
            map.push(Vec::new());

            for byte in line.bytes() {
                let item = match byte {
                    b'.' => Item::Empty,
                    b'S' => Item::Spawn,
                    b'^' => Item::Splitter,
                    _ => return Err("Invalid map".into()),
                };

                map[i].push(item);
            }
        }

        // TODO: Check that map dimensions are ok

        Ok(BeamSimulatorClassical {
            rows: map.len(),
            cols: map[0].len(),
            map,
            visited_positions: HashSet::new(),
            ans: 0,
        })
    }
}

impl Index<&Position> for BeamSimulatorClassical {
    type Output = Item;

    fn index(&self, index: &Position) -> &Self::Output {
        &self.map[index.row][index.col]
    }
}
