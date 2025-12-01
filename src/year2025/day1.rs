pub fn calculate_password(data: &str) -> (u32, u32) {
    let lines = data.lines();
    let mut dial = Dial::default();

    let mut passwd_part1: u32 = 0;
    for line in lines {
        dial.rotate(line);
        if dial.value == 0 {
            passwd_part1 += 1;
        }
    }

    let passwd_part2 = dial.visited_zeros;
    (passwd_part1, passwd_part2)
}

#[derive(Debug)]
struct Dial {
    value: i32,
    visited_zeros: u32,
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            value: 50,
            visited_zeros: 0,
        }
    }
}

impl Dial {
    fn rotate(&mut self, s: &str) {
        let amount: i32 = s[1..].parse().unwrap();
        assert!(amount > 0);

        if s.starts_with('L') {
            self.visited_zeros += amount as u32 / 100;
            let remainder = amount % 100;
            if (self.value != 0) && (remainder >= self.value) {
                self.visited_zeros += 1;
            }

            self.value = 99 - self.value;
            self.value = (self.value + amount) % 100;
            self.value = 99 - self.value;
        } else if s.starts_with('R') {
            self.visited_zeros += amount as u32 / 100;
            let remainder = amount % 100;
            if remainder > (99 - self.value) {
                self.visited_zeros += 1;
            }

            self.value = (self.value + amount) % 100;
        } else {
            panic!();
        }
    }
}
