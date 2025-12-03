use std::ops::RangeInclusive;

use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, &'static str> {
    let mut digits: Vec<usize> = Vec::new();

    for range_str in data.trim().split(',') {
        let range = parse_range(range_str);
        digits.append(&mut invalid_digits(range));
    }

    Solution::new(digits.iter().sum(), "TODO")
}

fn parse_range(range_str: &str) -> RangeInclusive<usize> {
    let mut iter = range_str.split('-');

    let start: usize = iter.next().unwrap().parse().unwrap();
    let end: usize = iter.next().unwrap().parse().unwrap();

    start..=end
}

pub(crate) fn invalid_digits(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut digits: Vec<usize> = Vec::new();

    for id in range {
        if !is_valid(id) {
            digits.push(id);
        }
    }

    digits
}

fn is_valid(id: usize) -> bool {
    let id_str = id.to_string();

    let len = id_str.len();
    let halflen = len / 2;

    if (len % 2) != 0 {
        return true;
    }

    id_str[..halflen] != id_str[halflen..]
}
