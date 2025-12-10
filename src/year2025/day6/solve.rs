use crate::solution::Solution;

pub fn solve(data: &str) -> Solution<i64, &'static str> {
    let mut lines = data.lines();

    let mut nums: Vec<Vec<i64>> = Vec::new();

    let line_first = lines.next().unwrap();
    for num_str in line_first.split_whitespace() {
        let num: i64 = num_str.parse().unwrap();
        nums.push(vec![num]);
    }

    let mut ops_str = "";
    for line in lines {
        if !line.trim_start().chars().next().unwrap().is_ascii_digit() {
            ops_str = line;
            break;
        }

        for (i, num_str) in line.split_whitespace().enumerate() {
            let num: i64 = num_str.parse().unwrap();
            nums[i].push(num);
        }
    }

    let mut part1: i64 = 0;
    for (i, op_str) in ops_str.split_whitespace().enumerate() {
        match op_str {
            "+" => part1 += nums[i].iter().sum::<i64>(),
            "*" => part1 += nums[i].iter().product::<i64>(),
            _ => panic!(),
        }
    }

    Solution {
        part1,
        part2: "TODO",
    }
}
