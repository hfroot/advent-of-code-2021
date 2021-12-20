use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
mod utils;

fn main() {
    println!("PART 1: {}", calculate_error("input.txt"));
    println!("PART 2: {}", part_2("input.txt"));
}

fn matches() -> HashMap<char, char> {
    let mut matches = HashMap::<char, char>::new();
    matches.insert('(', ')');
    matches.insert('[', ']');
    matches.insert('{', '}');
    matches.insert('<', '>');
    matches
}

fn invalid_char(line: String) -> Option<char> {
    let mut opening_brackets = vec![];
    let matches = matches();
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
    // could calculate as you go instead of this way
    let mut score = 0;
    for key in invalid_chars.keys() {
        score += points.get(key).unwrap() * invalid_chars.get(key).unwrap();
    }
    score
}

fn score_closing_seq(line: String) -> usize {
    let mut score = 0;
    let matches = matches();
    let mut opening_chars = "".to_string();
    for c in line.chars() {
        if matches.get(&c).is_some() {
            opening_chars.push(c);
        } else {
            opening_chars.pop(); // incomplete lines don't worry about syntax error
        }
    }
    let mut points = HashMap::<char, usize>::new();
    points.insert('(', 1);
    points.insert('[', 2);
    points.insert('{', 3);
    points.insert('<', 4);
    for c in opening_chars.chars().into_iter().rev() {
        score *= 5;
        score += points.get(&c).unwrap();
    }
    score
}

fn scores(filename: &str) -> Vec<usize> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut scores = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        // ignore invalid lines
        if invalid_char(l.clone()).is_none() {
            scores.push(score_closing_seq(l));
        }
    }
    scores
}

fn part_2(filename: &str) -> usize {
    utils::median(&mut scores(filename))
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
    #[test]
    fn determine_closing_string_score() {
        let line = "[({(<(())[]>[[{[]{<()<>>".to_string();
        assert_eq!(score_closing_seq(line), 288957);
    }
    #[test]
    fn scores_for_file() {
        assert_eq!(scores("test-input.txt"), vec![288957, 5566, 1480781, 995444, 294]);
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2("test-input.txt"), 288957);
    }
}