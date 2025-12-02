use std::str::FromStr;

pub struct Dial {
    value: u32,
    visited_zeros_strict: u32,
    visited_zeros_loose: u32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            value: 50,
            visited_zeros_strict: 0,
            visited_zeros_loose: 0,
        }
    }
}

impl Dial {
    pub fn visited_zeros_strict(&self) -> u32 {
        self.visited_zeros_strict
    }

    pub fn visited_zeros_loose(&self) -> u32 {
        self.visited_zeros_loose
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::Right(amount) => {
                self.visited_zeros_loose += amount / 100;
                let remainder = amount % 100;
                if (self.value != 0) && (remainder >= self.value) {
                    self.visited_zeros_loose += 1;
                }

                self.value = 99 - self.value;
                self.value = (self.value + amount) % 100;
                self.value = 99 - self.value;
            }
            Rotation::Left(amount) => {
                self.visited_zeros_loose += amount / 100;
                let remainder = amount % 100;
                if remainder > (99 - self.value) {
                    self.visited_zeros_loose += 1;
                }

                self.value = (self.value + amount) % 100;
            }
        }

        if self.value == 0 {
            self.visited_zeros_strict += 1;
        }
    }
}

pub enum Rotation {
    Right(u32),
    Left(u32),
}

impl FromStr for Rotation {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let amount: u32 = s[1..].parse()?;

        if s.starts_with('L') {
            Ok(Self::Left(amount))
        } else if s.starts_with('R') {
            Ok(Self::Right(amount))
        } else {
            let actual = match s.chars().next() {
                Some(c) => format!("{c}"),
                None => String::from(""),
            };
            Err(format!("expected 'L' or 'R', got '{actual}'").into())
        }
    }
}
