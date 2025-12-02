use std::fmt::Display;

pub struct Solution<T1, T2>
where
    T1: Display,
    T2: Display,
{
    part1: T1,
    part2: T2,
}

impl<T1, T2> Solution<T1, T2>
where
    T1: Display,
    T2: Display,
{
    pub fn new(part1: T1, part2: T2) -> Self {
        Self { part1, part2 }
    }
}

impl<T1, T2> Display for Solution<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Day1\n  - Part1: {}\n  - Part2: {}",
            self.part1, self.part2
        )
    }
}
