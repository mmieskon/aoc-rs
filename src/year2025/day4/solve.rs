use super::warehouse::Warehouse;
use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<u32, u32> {
    let mut warehouse: Warehouse = data.parse().unwrap();
    let mut indexes = warehouse.emptiables();

    let part1 = indexes.len().try_into().unwrap();

    loop {
        if indexes.is_empty() {
            break;
        }

        for (row, col) in indexes {
            warehouse.set_empty(row, col).unwrap();
        }

        indexes = warehouse.emptiables();
    }

    Solution {
        part1,
        part2: warehouse.emptied(),
    }
}
