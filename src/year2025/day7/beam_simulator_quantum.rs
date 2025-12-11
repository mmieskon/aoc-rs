use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Item {
    Empty,
    Spawn,
    Splitter,
}

#[derive(Debug)]
pub struct BeamSimulatorQuantum {
    map: Vec<Vec<Item>>,
    rows: usize,
    cols: usize,
    beams: Vec<(usize, u64)>, // Vec<(column, strength)>
    visited_splitters_count: u64,
}

impl BeamSimulatorQuantum {
    pub fn simulate(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if matches!(self.map[row][col], Item::Spawn) {
                    self.beams.push((col, 1));
                    break;
                }
            }
        }

        let mut new_beams: Vec<(usize, u64)> = Vec::new();
        for row in 0..self.rows {
            for (col, _) in self.map[row]
                .iter()
                .enumerate()
                .filter(|(_, item)| matches!(item, Item::Splitter))
            {
                if let Some(beam) = self.beams.iter_mut().find(|(c, _)| *c == col) {
                    beam.0 = beam.0.checked_sub(1).unwrap();
                    new_beams.push((col + 1, beam.1));
                }
            }

            self.beams.append(&mut new_beams);
            combine_beams(&mut self.beams);
        }

        for (_, strength) in &self.beams {
            self.visited_splitters_count += strength;
        }
    }

    pub fn visited_splitters_count(&self) -> u64 {
        self.visited_splitters_count
    }
}

fn combine_beams(beams: &mut Vec<(usize, u64)>) {
    let mut found = true;

    while found {
        found = false;

        'b: for i in 0..beams.len() {
            for j in (i + 1)..beams.len() {
                if beams[i].0 == beams[j].0 {
                    beams[i].1 += beams[j].1;
                    beams.remove(j);
                    found = true;
                    break 'b;
                }
            }
        }
    }
}

impl FromStr for BeamSimulatorQuantum {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<Item>> = Vec::new();

        for (i, line) in s.lines().enumerate() {
            map.push(Vec::new());

            for byte in line.bytes() {
                let item = match byte {
                    b'.' => Item::Empty,
                    b'S' => Item::Spawn,
                    b'^' => Item::Splitter,
                    _ => return Err("Invalid map".into()),
                };

                map[i].push(item);
            }
        }

        // TODO: Check that map dimensions are ok

        Ok(BeamSimulatorQuantum {
            rows: map.len(),
            cols: map[0].len(),
            map,
            beams: Vec::new(),
            visited_splitters_count: 0,
        })
    }
}
