use crate::Result;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Item {
    RollOfPaper,
    Empty,
}

#[derive(Debug)]
pub struct Warehouse {
    items: Vec<Item>,
    rows: usize,
    cols: usize,
    emptied: u32,
}

impl Warehouse {
    pub fn emptied(&self) -> u32 {
        self.emptied
    }

    pub fn get(&self, row: usize, col: usize) -> Option<Item> {
        if (row >= self.rows) || (col >= self.cols) {
            return None;
        }

        self.items.get(row * self.cols + col).copied()
    }

    pub fn get_neighbors(&self, row: usize, col: usize) -> [Option<Item>; 8] {
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

    pub fn set_empty(&mut self, row: usize, col: usize) -> Result<()> {
        if (row >= self.rows) || (col >= self.cols) {
            return Err("index out of bounds".into());
        }

        self.items[row * self.cols + col] = Item::Empty;
        self.emptied += 1;
        Ok(())
    }

    pub fn emptiables(&self) -> Vec<(usize, usize)> {
        let mut indexes: Vec<(usize, usize)> = Vec::new();

        for ((row, col), item) in self {
            if matches!(item, Item::Empty) {
                continue;
            }

            let mut adjacent_count = 0;
            for neighbor in self.get_neighbors(row, col) {
                if matches!(neighbor, Some(Item::RollOfPaper)) {
                    adjacent_count += 1;
                }
            }

            if adjacent_count < 4 {
                indexes.push((row, col));
            }
        }

        indexes
    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut warehouse = Self {
            items: Vec::new(),
            rows: 0,
            cols: 0,
            emptied: 0,
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

pub struct Iter<'a> {
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
