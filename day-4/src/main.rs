use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let (numbers, boards, tracking) = parse_input("input.txt");
    let score = get_score(numbers, tracking, boards);
    println!("SCORE: {}", score);
}

fn parse_input(
    filename: &str,
) -> (
    Vec<usize>,
    Vec<([usize; 10], usize)>,
    HashMap<usize, Vec<[usize; 3]>>, // instead of array because hard to initialize array with empty vectors
) {
    let file = File::open(filename).expect("file not found :(");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let numberline = lines.next().unwrap().unwrap();
    let numbers = numberline
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut boards = Vec::<([usize; 10], usize)>::new();
    lines.next(); // ignore blank line after number line
    let mut sum = 0;
    let mut tracking = HashMap::<usize, Vec<[usize; 3]>>::new();
    let mut y = 0;
    for line in lines {
        let line = line.unwrap();
        if line == "" {
            // this doesn't actually work unless there are two blank lines at the end of file
            boards.push(([0; 10], sum));
            sum = 0;
            y = 0;
        } else {
            let numbers = line.split_whitespace().collect::<Vec<&str>>();
            let mut x = 0;
            for n in numbers {
                let number = n.parse::<usize>().unwrap();
                sum += number;
                tracking
                    .entry(number)
                    .or_insert(Vec::<[usize; 3]>::new())
                    .push([boards.len(), x, y]);
                x += 1;
            }
            y += 1;
        }
    }
    (numbers, boards, tracking)
}

fn get_score(
    numbers: Vec<usize>,
    tracking: HashMap<usize, Vec<[usize; 3]>>,
    mut boards: Vec<([usize; 10], usize)>,
) -> usize {
    for number in numbers {
        let positions = tracking.get(&number);
        if let Some(positions) = positions {
            for position in positions {
                let board_id = position[0];
                boards[board_id].0[position[1]] += 1;
                // offset by size of board which is 5
                let position_y = position[2] + 5;
                boards[board_id].0[position_y] += 1;
                boards[board_id].1 -= number; // in the end, it is sum of unmarked numbers
                if boards[board_id].0[position[1]] >= 5 || boards[board_id].0[position_y] >= 5 {
                    println!("{:?}", board_id);
                    println!("{:?}", boards[board_id].1);
                    println!("{:?}", number);
                    return boards[board_id].1 * number;
                }
            }
        }
    }
    0 // fallback
}

#[cfg(test)]
mod tests {
    use crate::get_score;
    use crate::parse_input;
    #[test]
    fn input_parsed_correctly() {
        let (numbers, boards, tracking) = parse_input("test-input.txt");
        assert_eq!(numbers, [11, 12, 45, 14, 24, 15, 13]);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[1], ([0; 10], 325));
        assert_eq!(tracking[&1], [[1, 0, 0], [2, 2, 1]]);
    }

    #[test]
    fn correct_score_returned() {
        // could write out the data structures myself, but this is easier
        let (numbers, boards, tracking) = parse_input("test-input.txt");
        assert_eq!(get_score(numbers, tracking, boards), 3068);
    }
}
