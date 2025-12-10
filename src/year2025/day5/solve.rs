use std::ops::RangeInclusive;

use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, usize> {
    Solution {
        part1: solve_part1(data),
        part2: solve_part2(data),
    }
}

fn solve_part1(data: &str) -> usize {
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

    let ids = lines[empty_idx + 1..]
        .iter()
        .map(|&line| line.parse::<usize>().unwrap());

    let mut fresh_ids: usize = 0;

    for id in ids {
        for range in &ranges {
            if range.contains(&id) {
                fresh_ids += 1;
                break;
            }
        }
    }

    fresh_ids
}

fn solve_part2(data: &str) -> usize {
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();

    for line in data.lines() {
        if line.is_empty() {
            break;
        }

        let splitted = line.split_once('-').unwrap();
        let start: usize = splitted.0.parse().unwrap();
        let end: usize = splitted.1.parse().unwrap();

        ranges.push(start..=end);
    }

    while let Some((idx1, idx2)) = find_intersecting(&ranges) {
        combine_intersecting(&mut ranges, idx1, idx2);
    }

    ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

fn intersects(range1: &RangeInclusive<usize>, range2: &RangeInclusive<usize>) -> bool {
    ((range2.start() < range1.start()) && (range2.end() > range1.end()))
        || range1.contains(range2.start())
        || range1.contains(range2.end())
}

fn find_intersecting(ranges: &[RangeInclusive<usize>]) -> Option<(usize, usize)> {
    for i in 0..ranges.len() {
        for j in (i + 1)..ranges.len() {
            if intersects(&ranges[i], &ranges[j]) {
                return Some((i, j));
            }
        }
    }

    None
}

fn combine_intersecting(ranges: &mut Vec<RangeInclusive<usize>>, idx1: usize, idx2: usize) {
    let start = *ranges[idx1].start().min(ranges[idx2].start());
    let end = *ranges[idx1].end().max(ranges[idx2].end());

    ranges[idx1] = start..=end;
    ranges.remove(idx2);
}
