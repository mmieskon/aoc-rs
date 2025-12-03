use std::fmt::Display;

pub struct Solution<T1, T2>
where
    T1: Display,
    T2: Display,
{
    pub part1: T1,
    pub part2: T2,
}

impl<T1, T2> Display for Solution<T1, T2>
where
    T1: Display,
    T2: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  - Part1: {}\n  - Part2: {}", self.part1, self.part2)
    }
}
