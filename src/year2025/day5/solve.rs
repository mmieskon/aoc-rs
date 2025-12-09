use std::ops::RangeInclusive;

use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, &str> {
    let lines: Vec<&str> = data.lines().collect();

    let mut empty_idx = None;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            empty_idx = Some(i);
            break;
        }
    }
    let empty_idx = empty_idx.unwrap();

    let ranges: Vec<RangeInclusive<usize>> = lines[..empty_idx]
        .iter()
        .map(|&line| line.split_once('-').unwrap())
        .map(|(start, end)| start.parse().unwrap()..=end.parse().unwrap())
        .collect();

    let ids: Vec<usize> = lines[empty_idx + 1..]
        .iter()
        .map(|&line| line.parse().unwrap())
        .collect();

    let mut fresh_ids: usize = 0;

    for id in &ids {
        for range in &ranges {
            if range.contains(id) {
                fresh_ids += 1;
                break;
            }
        }
    }

    Solution {
        part1: fresh_ids,
        part2: "TODO",
    }
}
