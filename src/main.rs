use std::fmt::Display;

struct Dial {
    value: i32,
}

impl Dial {
    fn new() -> Self {
        Self { value: 50 }
    }

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

fn main() {
    let path: String = std::env::args().skip(1).take(1).collect();
    let content = std::fs::read_to_string(&path).unwrap();
    let lines = content
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim_end());

    let mut dial = Dial::new();

    let mut ans: u32 = 0;
    for line in lines {
        dial.rotate(line);
        if dial.value == 0 {
            ans += 1;
        }
    }

    println!("{ans}");
}
