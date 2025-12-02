use std::ops::RangeInclusive;

use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, &'static str> {
    let mut digits: Vec<usize> = Vec::new();

    for range_str in data.trim().split(',') {
        let range = parse_range(range_str);
        digits.append(&mut invalid_digits(range));
    }

    Solution::new(2, digits.iter().sum(), "TODO")
}

fn parse_range(range_str: &str) -> RangeInclusive<usize> {
    let mut iter = range_str.split('-');

    let start: usize = iter.next().unwrap().parse().unwrap();
    let end: usize = iter.next().unwrap().parse().unwrap();

    start..=end
}

fn invalid_digits(range: RangeInclusive<usize>) -> Vec<usize> {
    let mut digits: Vec<usize> = Vec::new();

    for id in range {
        if is_invalid(id) {
            digits.push(id);
        }
    }

    digits
}

fn is_invalid(id: usize) -> bool {
    let id_str = id.to_string();

    // NOTE: Each char is one byte because all of the chars are digits
    for window_size in 1..(id_str.len() / 2) {
        if repeats(&id_str, window_size) {
            return true;
        }
    }

    false
}

fn repeats(byte_str: &str, window_size: usize) -> bool {
    let mut peekable = byte_str.byte_window_iter(window_size).peekable();

    while let Some(current) = peekable.next() {
        if peekable.peek().is_some_and(|&peeked| peeked == current) {
            return true;
        }
    }

    false
}

// NOTE: Assumes that byte_str only contains characters that take one byte each
struct ByteWindowIter<'a> {
    byte_str: &'a str,
    idx: usize,
    window_size: usize,
}

impl<'a> Iterator for ByteWindowIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.idx + self.window_size) > self.byte_str.len() {
            return None;
        }

        let start = self.idx;
        let end = start + self.window_size;

        self.idx += self.window_size;
        Some(&self.byte_str[start..end])
    }
}

trait ByteWindowIterator {
    fn byte_window_iter(&self, window_size: usize) -> ByteWindowIter;
}

impl ByteWindowIterator for &str {
    fn byte_window_iter(&self, window_size: usize) -> ByteWindowIter {
        ByteWindowIter {
            byte_str: self,
            idx: 0,
            window_size,
        }
    }
}
