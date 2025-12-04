use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, usize> {
    let mut total_joltage = (0, 0);

    for line in data.lines() {
        total_joltage.0 += joltage(line, 2);
        total_joltage.1 += joltage(line, 12);
    }

    Solution {
        part1: total_joltage.0,
        part2: total_joltage.1,
    }
}

pub(crate) fn joltage(line: &str, digit_count: usize) -> usize {
    let mut digits: Vec<u8> = Vec::new();
    let bytes: Vec<_> = line.bytes().enumerate().collect();

    let mut start = 0;
    for i in 0..digit_count {
        let end = line.len() - (digit_count - i);

        let (idx, byte) = max_first(&bytes[start..=end]);
        digits.push(byte);
        start = idx + 1;
    }

    String::from_utf8_lossy(&digits).parse().unwrap()
}

fn max_first(bytes: &[(usize, u8)]) -> (usize, u8) {
    *bytes.iter().rev().max_by(|x, y| x.1.cmp(&y.1)).unwrap()
}
