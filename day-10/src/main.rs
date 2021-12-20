use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    println!("PART 1: {}", calculate_error("input.txt"));
}

fn invalid_char(line: String) -> Option<char> {
    let mut opening_brackets = vec![];
    let mut matches = HashMap::<char, char>::new();
    matches.insert('(', ')');
    matches.insert('[', ']');
    matches.insert('{', '}');
    matches.insert('<', '>');
    for c in line.chars() {
        if matches.get(&c).is_some() {
            opening_brackets.push(c);
        } else {
            let last_open = opening_brackets.pop();
            if last_open.is_none() || c != *matches.get(&(last_open.unwrap())).unwrap() {
                return Some(c);
            }
        }
    }
    None
}

fn calculate_error(filename: &str) -> usize {
    let mut points = HashMap::<char, usize>::new();
    points.insert(')', 3);
    points.insert(']', 57);
    points.insert('}', 1197);
    points.insert('>', 25137);
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut invalid_chars = HashMap::<char, usize>::new();
    invalid_chars.insert(')', 0);
    invalid_chars.insert(']', 0);
    invalid_chars.insert('}', 0);
    invalid_chars.insert('>', 0);
    for line in reader.lines() {
        if let Some(c) = invalid_char(line.unwrap()) {
            *invalid_chars.get_mut(&c).unwrap() += 1;
        }
    }
    let mut score = 0;
    for key in invalid_chars.keys() {
        score += points.get(key).unwrap() * invalid_chars.get(key).unwrap();
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn find_invalid_char() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>".to_string();
        assert_eq!(invalid_char(line), Some('}'));
    }
    #[test]
    fn correct_error_score() {
        assert_eq!(calculate_error("test-input.txt"), 26397);
    }
}