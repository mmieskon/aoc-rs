use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<u64, u64> {
    Solution {
        part1: solve_part1(data),
        part2: solve_part2(data),
    }
}

fn solve_part1(data: &str) -> u64 {
    let mut lines = data.lines();

    let mut nums: Vec<Vec<u64>> = Vec::new();

    let line_first = lines.next().unwrap();
    for num_str in line_first.split_whitespace() {
        let num: u64 = num_str.parse().unwrap();
        nums.push(vec![num]);
    }

    let mut ops_str = "";
    for line in lines {
        if !line.trim_start().chars().next().unwrap().is_ascii_digit() {
            ops_str = line;
            break;
        }

        for (i, num_str) in line.split_whitespace().enumerate() {
            let num: u64 = num_str.parse().unwrap();
            nums[i].push(num);
        }
    }

    let mut ret: u64 = 0;
    for (i, op_str) in ops_str.split_whitespace().enumerate() {
        match op_str {
            "+" => ret += nums[i].iter().sum::<u64>(),
            "*" => ret += nums[i].iter().product::<u64>(),
            _ => panic!(),
        }
    }

    ret
}

fn solve_part2(data: &str) -> u64 {
    let bytes: Vec<Vec<u8>> = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.bytes().collect())
        .collect();

    let rows = bytes.len();
    let cols = bytes[0].len();

    let mut ret = 0;
    let mut digits = vec![0u8; rows - 1];
    let mut nums: Vec<u64> = Vec::new();

    for col in (0..cols).rev() {
        for (row, digit_byte) in digits.iter_mut().enumerate() {
            *digit_byte = bytes[row][col];
        }

        let digits_str = str::from_utf8(&digits).unwrap().trim();
        if digits_str.is_empty() {
            nums.clear();
            continue;
        }

        let num: u64 = digits_str.parse().unwrap();
        nums.push(num);

        let op = bytes[rows - 1][col];

        match op {
            b'+' => ret += nums.iter().sum::<u64>(),
            b'*' => ret += nums.iter().product::<u64>(),
            _ => continue,
        }
    }

    ret
}
