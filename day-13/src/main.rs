use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

#[derive(Debug, PartialEq)]
enum Axis {
    X,
    Y,
}

struct Coordinate {
    x: u16,
    y: u16,
}

fn new_coordinate(original_value: &u16, constant: &u16) -> u16 {
    original_value - ((original_value - constant) * 2)
}

struct Instruction {
    axis: Axis,
    constant: u16,
}

#[derive(Clone)]
struct Line {
    holes: HashSet<u16>,
}

impl Line {
    fn has_hole(&self, dot: &u16) -> bool {
        self.holes.contains(dot)
    }
    fn hole_count(&self) -> usize {
        self.holes.len()
    }
    fn add_holes(&mut self, other_line: Line) {
        for hole in other_line.holes.iter() {
            self.holes.insert(*hole);
        }
    }
}

fn new_line() -> Line {
    Line {
        holes: HashSet::new(),
    }
}

struct TransparentPaper {
    // vertical_lines: HashMap<u16, Line>,
    horizontal_lines: HashMap<u16, Line>,
    instructions: Vec<Instruction>,
    max_y: u16,
    max_x: u16,
}

impl TransparentPaper {
    fn fold(&mut self, instructions_limit: Option<usize>) {
        let limit = instructions_limit.unwrap_or(self.instructions.len());
        for i in 0..limit {
            let constant = self.instructions[i].constant;
            if self.instructions[i].axis == Axis::Y {
                (*self).fold_horizontally(constant);
            } else {
                (*self).fold_vertically(constant)
            }
        }
    }
    fn fold_horizontally(&mut self, constant: u16) {
        for y in self.horizontal_lines.clone().into_keys() {
            if y > constant {
                let lines = self.horizontal_lines.clone();
                let lower_line = lines.get(&y).unwrap();
                let new_y = new_coordinate(&y, &constant);
                if self.horizontal_lines.contains_key(&new_y) {
                    self.horizontal_lines.entry(new_y).and_modify(|line| (*line).add_holes(lower_line.clone()));
                } else {
                    self.horizontal_lines.insert(new_y, lower_line.clone());
                }
                self.horizontal_lines.remove(&y);
            }
        }
        self.max_y -= constant;
    }
    fn fold_vertically(&mut self, constant: u16) {
        for y in self.horizontal_lines.clone().into_keys() {
            for x in self.horizontal_lines.get(&y).unwrap().holes.clone().iter() {
                if x > &constant {
                    let new_x = new_coordinate(&x, &constant);
                    (*self).horizontal_lines.entry(y.clone()).and_modify(|line| { (*line).holes.remove(x); });
                    (*self).horizontal_lines.entry(y).and_modify(|line| { (*line).holes.insert(new_x); });
                }
            }
        }
        self.max_x -= constant;
    }
    fn hole_count(&self) -> usize {
        let mut count = 0;
        for line in self.horizontal_lines.clone().into_values() {
            count += line.hole_count();
        }
        count
    }
    fn has_hole(&self, point: &Coordinate) -> bool {
        if let Some(line) = self.horizontal_lines.get(&point.y) {
            return (*line).has_hole(&point.x);
        }
        false
    }
}

fn read_paper(filename: &str) -> TransparentPaper {
    let file = File::open(filename).expect("oh no");
    let reader = BufReader::new(file);
    let mut horizontal_lines = HashMap::new();
    // let mut horizontal_lines = HashMap::new();
    let mut instructions = Vec::new();
    let mut finished_holes = false;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            finished_holes = true;
        } else if !finished_holes {
            let coordinates = line
                .split(',')
                .map(|x| x.parse::<u16>().unwrap())
                .collect::<Vec<u16>>();
            if coordinates[1] > max_y {
                max_y = coordinates[1].clone();
            }
            let line = horizontal_lines.entry(coordinates[1]).or_insert(new_line());
            if coordinates[0] > max_x {
                max_x = coordinates[0].clone();
            }
            (*line).holes.insert(coordinates[0]);
            // horizontal_lines.insert(coordinates[1], coordinates[0]);
        } else {
            let instruction = line.split("fold along ").collect::<Vec<&str>>();
            let instruction = instruction[1];
            let instruction = instruction.split('=').collect::<Vec<&str>>();
            let mut axis = Axis::Y;
            if instruction[0] == "x" {
                axis = Axis::X;
            }
            let constant = instruction[1].parse::<u16>().unwrap();
            instructions.push(Instruction{ axis, constant });
        }
    }
    TransparentPaper { horizontal_lines, instructions, max_x, max_y }
}

impl std::fmt::Display for TransparentPaper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = "".to_string();
        for y in 0..(self.max_y) {
            for x in 0..(self.max_x) {
                let mut dot = " ";
                if self.has_hole(&Coordinate{x,y}) {
                    dot = "8";
                }
                string = format!("{}{}", string, dot);
            }
            string = format!("{}\n", string);
        }
        write!(f, "{}", string)
    }
}

fn part_1() {
    let mut paper = read_paper("input.txt");
    paper.fold(Some(1));
    println!("PART 1: {}", paper.hole_count());
}

fn part_2() {
    let mut paper = read_paper("input.txt");
    paper.fold(None);
    println!("PART 2-------------");
    println!("{}", paper);
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_reading() {
        let paper = read_paper("test-input.txt");
        assert_eq!(paper.hole_count(), 18);
        assert!(paper.has_hole(&Coordinate{x: 10, y: 4}));
        assert_eq!(paper.instructions.len(), 2);
        assert_eq!(paper.instructions[0].axis, Axis::Y);
        assert_eq!(paper.instructions[0].constant, 7 as u16);
        assert_eq!(paper.max_y, 14);
        assert_eq!(paper.max_x, 10);
    }
    #[test]
    fn test_folding_1_step() {
        let mut paper = read_paper("test-input.txt");
        paper.fold(Some(1));
        assert_eq!(paper.hole_count(), 17);
    }
    #[test]
    fn test_folding() {
        let mut paper = read_paper("test-input.txt");
        paper.fold(None);
        assert_eq!(paper.hole_count(), 16);
        assert_eq!(paper.max_y, 7);
        println!("{}", paper);
        assert_eq!(paper.max_x, 5);
    }
}