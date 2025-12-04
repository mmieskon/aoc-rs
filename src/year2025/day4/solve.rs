use std::str::FromStr;

use crate::solution::Solution;

#[derive(Debug, Clone, Copy)]
enum Item {
    RollOfPaper,
    Empty,
}

#[derive(Debug)]
struct Warehouse {
    items: Vec<Item>,
    rows: usize,
    cols: usize,
}

impl Warehouse {
    fn get(&self, row: usize, col: usize) -> Option<Item> {
        if (row >= self.rows) || (col >= self.cols) {
            return None;
        }

        self.items.get(row * self.cols + col).copied()
    }

    fn get_neighbors(&self, row: usize, col: usize) -> [Option<Item>; 8] {
        let mut neighbors = [None; 8];
        let row: isize = row.try_into().unwrap();
        let col: isize = col.try_into().unwrap();

        let mut idx = 0;
        for i in (row - 1)..=(row + 1) {
            for j in (col - 1)..=(col + 1) {
                if (i, j) == (row, col) {
                    continue;
                }

                if (i >= 0) && (j >= 0) {
                    let i = i.try_into().unwrap();
                    let j = j.try_into().unwrap();
                    neighbors[idx] = self.get(i, j);
                }

                idx += 1;
            }
        }

        neighbors
    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut warehouse = Self {
            items: Vec::new(),
            rows: 0,
            cols: 0,
        };

        warehouse.cols = s.lines().next().unwrap().len();
        warehouse.rows = 0;

        for line in s.lines() {
            for byte in line.bytes() {
                if byte == b'@' {
                    warehouse.items.push(Item::RollOfPaper);
                } else {
                    warehouse.items.push(Item::Empty);
                }
            }

            warehouse.rows += 1;
        }

        Ok(warehouse)
    }
}

struct Iter<'a> {
    warehouse: &'a Warehouse,
    idx: (usize, usize),
}

impl Iterator for Iter<'_> {
    type Item = ((usize, usize), Item);

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.idx.0;
        let col = self.idx.1;

        let item = self.warehouse.get(row, col)?;

        self.idx.1 += 1;
        if self.idx.1 >= self.warehouse.cols {
            self.idx.0 += 1;
            self.idx.1 = 0;
        }

        Some(((row, col), item))
    }
}

impl<'a> IntoIterator for &'a Warehouse {
    type Item = ((usize, usize), Item);

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            warehouse: self,
            idx: (0, 0),
        }
    }
}

pub fn solve(data: &str) -> Solution<u32, &'static str> {
    let mut rolls_of_paper = 0;

    let warehouse: Warehouse = data.parse().unwrap();

    for ((row, col), item) in &warehouse {
        if matches!(item, Item::Empty) {
            continue;
        }

        let mut adjacent_count = 0;
        for neighbor in warehouse.get_neighbors(row, col) {
            if matches!(neighbor, Some(Item::RollOfPaper)) {
                adjacent_count += 1;
            }
        }

        if adjacent_count < 4 {
            rolls_of_paper += 1;
        }
    }

    Solution {
        part1: rolls_of_paper,
        part2: "TODO",
    }
}
