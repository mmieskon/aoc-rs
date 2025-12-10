use std::ops::RangeInclusive;

pub(crate) fn solve_part1(data: &str) -> usize {
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
