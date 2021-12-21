use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
mod point;

fn main() {
    part_1();
}

#[derive(Copy, Clone)]
struct Octopus {
    energy: u8,
    location: point::Point,
}

fn spawn_octopus(idx: &u8, energy: u8) -> Octopus {
    Octopus {
        energy,
        location: point::point_from_idx(idx),
    }
}

impl Octopus {
    fn increment_without_flash(&mut self, cavern: &mut Cavern) {
        self.energy += 1;
        cavern.update_octopus(*self);
    }

    fn increment(&mut self, cavern: &mut Cavern) {
        self.energy += 1;
        cavern.update_octopus(*self);
        self.flash_maybe(cavern);
    }

    fn flash_maybe(&mut self, cavern: &mut Cavern) {
        if self.energy == 10 {
            cavern.flash_count += 1;
            for spot in self.location.neighbours {
                if let Some(idx) = spot {
                    if let Some(mut neighbour) = cavern.get_octopus(idx) {
                        neighbour.increment(cavern);
                    }
                }
            }
        }
    }

    fn reset_flash(&mut self, cavern: &mut Cavern) {
        if self.energy > 9 {
            self.energy = 0;
            cavern.update_octopus(*self);
        }
    }
}

struct Cavern {
    // probably bad that Octopus uses Cavern and Cavern uses Octopus
    grid: [Option<Octopus>;100],
    flash_count: usize,
    current_octopus_count: u8, // for initialisation
}

fn build_cavern() -> Cavern {
    Cavern {
        grid: [None;100],
        flash_count: 0,
        current_octopus_count: 0,
    }
}

impl Cavern {
    fn add(&mut self, octopus: Octopus) {
        // assumes good data: not too many octopuses, no duplicates
        self.grid[self.current_octopus_count as usize] = Some(octopus);
        self.current_octopus_count += 1;
    }

    fn steps(&mut self, count: usize) {
        for _ in 0..count {
            self.increment();
            self.flash();
            self.reset_flashed();
        }
    }

    fn increment(&mut self) {
        for spot in self.grid.into_iter() {
            if let Some(mut octopus) = spot {
                octopus.increment_without_flash(self);
            }
        }
    }

    fn flash(&mut self) {
        // don't use iter_mut() because need to borrow mutable self for increment later
        for spot in self.grid {
            if let Some(mut octopus) = spot {
                octopus.flash_maybe(self);
            }
        }
    }

    fn reset_flashed(&mut self) {
        for spot in self.grid {
            if let Some(mut octopus) = spot {
                octopus.reset_flash(self);
            }
        }
    }

    fn get_octopus(&mut self, idx: u8) -> Option<Octopus> {
        let idx = idx as usize;
        if idx < self.grid.len() {
            return self.grid[idx];
        }
        None
    }

    // I feel like the existance of this shouldn't be necessary if I knew how to pass by reference properly
    fn update_octopus(&mut self, octopus: Octopus) {
        self.grid[point::point_to_idx(octopus.location) as usize] = Some(octopus);
    }
}

fn populate_cavern(filename: &str) -> Cavern {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut cavern = build_cavern();
    let mut idx = 0;
    for line in reader.lines() {
        for c in line.unwrap().chars() {
            let energy = c.to_digit(10).unwrap() as u8;
            cavern.add(spawn_octopus(&idx, energy));
            idx += 1;
        }
    }
    cavern
}

// this was very useful for debug purposes
impl std::fmt::Display for Cavern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = "".to_string();
        for (idx, octopus) in self.grid.iter().enumerate() {
            string = format!("{}{} ", string, octopus.unwrap().energy);
            if (idx+1) % 10 == 0 {
                string = format!("{}\n", string);
            }
        }
        write!(f, "{}", string)
    }
}

fn part_1() {
    let mut cavern = populate_cavern("input.txt");
    cavern.steps(100);
    println!("PART 1: {}", cavern.flash_count);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn mini_test_1_steps() {
        let mut cavern = populate_cavern("mini-test-input.txt");
        cavern.steps(1);
        assert_eq!(cavern.flash_count, 2);
        assert_eq!(cavern.grid[3].unwrap().energy, 4);
    }
    #[test]
    fn mini_test_2_steps() {
        let mut cavern = populate_cavern("mini-test-input.txt");
        cavern.steps(2);
        assert_eq!(cavern.flash_count, 2);
        assert_eq!(cavern.grid[3].unwrap().energy, 5);
    }
    #[test]
    fn test_2_steps() {
        // check grid for part 1 after two steps
        let mut cavern = populate_cavern("test-input.txt");
        cavern.steps(2);
        assert_eq!(cavern.flash_count, 35);
    }
    #[test]
    fn part_1() {
        let mut cavern = populate_cavern("test-input.txt");
        cavern.steps(100);
        assert_eq!(cavern.flash_count, 1656);
    }
}