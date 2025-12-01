pub fn calculate_password(data: &str) -> (u32, u32) {
    let lines = data.lines();
    let mut dial = Dial::default();

    for line in lines {
        dial.rotate(line);
    }

    (dial.visited_zeros_strict, dial.visited_zeros_loose)
}

#[derive(Debug)]
struct Dial {
    value: i32,
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
    fn rotate(&mut self, s: &str) {
        let amount: i32 = s[1..].parse().unwrap();
        assert!(amount > 0);

        if s.starts_with('L') {
            self.visited_zeros_loose += amount as u32 / 100;
            let remainder = amount % 100;
            if (self.value != 0) && (remainder >= self.value) {
                self.visited_zeros_loose += 1;
            }

            self.value = 99 - self.value;
            self.value = (self.value + amount) % 100;
            self.value = 99 - self.value;
        } else if s.starts_with('R') {
            self.visited_zeros_loose += amount as u32 / 100;
            let remainder = amount % 100;
            if remainder > (99 - self.value) {
                self.visited_zeros_loose += 1;
            }

            self.value = (self.value + amount) % 100;
        } else {
            panic!();
        }

        if self.value == 0 {
            self.visited_zeros_strict += 1;
        }
    }
}
