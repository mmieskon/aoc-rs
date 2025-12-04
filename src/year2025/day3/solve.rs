use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<usize, &'static str> {
    let mut total_joltage = 0;

    for line in data.lines() {
        total_joltage += joltage(line);
    }

    Solution {
        part1: total_joltage,
        part2: "TODO",
    }
}

pub(crate) fn joltage(line: &str) -> usize {
    let last_idx = line.len() - 1;

    let max_byte = line.bytes().max().unwrap();
    let idx = line
        .bytes()
        .enumerate()
        .find(|(_, val)| *val == max_byte)
        .map(|tuple| tuple.0)
        .unwrap();

    let ch1;
    let ch2;
    if idx == last_idx {
        let max_byte_first = line[..last_idx].bytes().max().unwrap();
        ch1 = char::from(max_byte_first);
        ch2 = char::from(max_byte);
    } else {
        let max_byte_second = line[(idx + 1)..].bytes().max().unwrap();
        ch1 = char::from(max_byte);
        ch2 = char::from(max_byte_second);
    }

    format!("{ch1}{ch2}").parse().unwrap()
}
