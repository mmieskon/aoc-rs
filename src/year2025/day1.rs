use std::fmt::Display;

pub fn calculate_password(data: &str) -> u32 {
    let lines = data
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim_end());

    let mut dial = Dial::default();

    let mut password: u32 = 0;
    for line in lines {
        dial.rotate(line);
        if dial.value == 0 {
            password += 1;
        }
    }

    password
}

struct Dial {
    value: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { value: 50 }
    }
}

impl Dial {
    fn rotate(&mut self, s: &str) {
        let amount: i32 = s[1..].parse().unwrap();

        if s.starts_with('L') {
            self.value = 99 - self.value;
            self.value = (self.value + amount) % 100;
            self.value = 99 - self.value;
        } else if s.starts_with('R') {
            self.value = (self.value + amount) % 100;
        } else {
            panic!();
        }
    }
}

impl Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
