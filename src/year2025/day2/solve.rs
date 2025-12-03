use std::ops::RangeInclusive;

use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, usize> {
    let mut digits1: Vec<usize> = Vec::new();
    let mut digits2: Vec<usize> = Vec::new();

    for range_str in data.trim().split(',') {
        let range = parse_range(range_str);
        let (mut invalid1, mut invalid2) = invalid_digits(range);

        digits1.append(&mut invalid1);
        digits2.append(&mut invalid2);
    }

    Solution {
        part1: digits1.iter().sum(),
        part2: digits2.iter().sum(),
    }
}

fn parse_range(range_str: &str) -> RangeInclusive<usize> {
    let mut iter = range_str.split('-');

    let start: usize = iter.next().unwrap().parse().unwrap();
    let end: usize = iter.next().unwrap().parse().unwrap();

    start..=end
}

pub(crate) fn invalid_digits(range: RangeInclusive<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut digits1: Vec<usize> = Vec::new();
    let mut digits2: Vec<usize> = Vec::new();

    for id in range {
        if !is_valid1(id) {
            digits1.push(id);
        }
        if !is_valid2(id) {
            digits2.push(id);
        }
    }

    (digits1, digits2)
}

fn is_valid1(id: usize) -> bool {
    let id_string = id.to_string();

    let len = id_string.len();
    let halflen = len / 2;

    if (len % 2) != 0 {
        return true;
    }

    id_string[..halflen] != id_string[halflen..]
}

fn is_valid2(id: usize) -> bool {
    let id_string = id.to_string();
    let id_str = id_string.as_str();
    let len = id_str.len(); // NOTE: Digits always take only one byte as char

    for window_size in 1..=(len / 2) {
        if len % window_size != 0 {
            continue;
        }

        let mut peekable = id_str.byte_window_iter(window_size).peekable();
        let mut current_repeats = true;

        while let Some(current) = peekable.next() {
            if let Some(&peeked) = peekable.peek()
                && (peeked != current)
            {
                current_repeats = false;
                break;
            }
        }

        if current_repeats {
            return false;
        }
    }

    true
}

#[derive(Debug)]
struct ByteWindowIter<'a> {
    byte_str: &'a str,
    idx: usize,
    window_size: usize,
}

impl<'a> Iterator for ByteWindowIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx + self.window_size > self.byte_str.len() {
            return None;
        }

        let start = self.idx;
        let end = start + self.window_size;

        self.idx += self.window_size;

        Some(&self.byte_str[start..end])
    }
}

trait AsByteWindowIter {
    fn byte_window_iter(&self, window_size: usize) -> ByteWindowIter;
}

impl AsByteWindowIter for &str {
    fn byte_window_iter(&self, window_size: usize) -> ByteWindowIter {
        ByteWindowIter {
            byte_str: self,
            idx: 0,
            window_size,
        }
    }
}
