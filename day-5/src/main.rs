use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let overlaps = get_overlaps("input.txt");
    println!("PART 1: {:?}", overlaps);
    let overlaps = get_overlaps_with_diagonal("input.txt");
    println!("PART 2: {:?}", overlaps);
}

fn get_overlaps(filename: &str) -> usize {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut board = HashMap::<usize, HashMap<usize, usize>>::new();
    let mut overlaps = 0;
    for line in lines {
        let line = line.unwrap();
        let coordinates = line.split(" -> ").collect::<Vec<&str>>();
        let c1 = coordinates[0]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let c1 = (c1[0], c1[1]);
        let c2 = coordinates[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let c2 = (c2[0], c2[1]);
        let is_vertical = c1.0 == c2.0;
        let is_horizonal = c1.1 == c2.1;
        if is_horizonal {
            let mut first = c1.0;
            let mut last = c2.0;
            if last < first {
                first = c2.0;
                last = c1.0;
            }
            let mut idx = first;
            while idx < last + 1 {
                *board
                    .entry(c1.1)
                    .or_insert(HashMap::<usize, usize>::new())
                    .entry(idx)
                    .or_insert(0) += 1;
                if *board.get(&c1.1).unwrap().get(&idx).unwrap() == 2 {
                    overlaps += 1;
                }
                idx += 1;
            }
        } else if is_vertical {
            let mut first = c1.1;
            let mut last = c2.1;
            if last < first {
                first = c2.1;
                last = c1.1;
            }
            let mut idx = first;
            while idx < last + 1 {
                *board
                    .entry(idx)
                    .or_insert(HashMap::<usize, usize>::new())
                    .entry(c1.0)
                    .or_insert(0) += 1;
                if *board.get(&idx).unwrap().get(&c1.0).unwrap() == 2 {
                    overlaps += 1;
                }
                idx += 1;
            }
        }
        // only consider horizontal and vertical lines
    }
    overlaps
}

fn get_overlaps_with_diagonal(filename: &str) -> usize {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut board = HashMap::<i32, HashMap<usize, usize>>::new();
    let mut overlaps = 0;
    for line in lines {
        let line = line.unwrap();
        let coordinates = line.split(" -> ").collect::<Vec<&str>>();
        let c1 = coordinates[0]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let c1 = (c1[0], c1[1] as i32);
        let c2 = coordinates[1]
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let c2 = (c2[0], c2[1] as i32);
        let is_vertical = c1.0 == c2.0;
        let is_horizonal = c1.1 == c2.1;
        if is_horizonal {
            let mut first = c1.0;
            let mut last = c2.0;
            if last < first {
                first = c2.0;
                last = c1.0;
            }
            let mut idx = first;
            while idx < last + 1 {
                *board
                    .entry(c1.1)
                    .or_insert(HashMap::<usize, usize>::new())
                    .entry(idx)
                    .or_insert(0) += 1;
                if *board.get(&c1.1).unwrap().get(&idx).unwrap() == 2 {
                    overlaps += 1;
                }
                idx += 1;
            }
        } else if is_vertical {
            let mut first = c1.1;
            let mut last = c2.1;
            if last < first {
                first = c2.1;
                last = c1.1;
            }
            let mut idx = first;
            while idx < last + 1 {
                *board
                    .entry(idx)
                    .or_insert(HashMap::<usize, usize>::new())
                    .entry(c1.0)
                    .or_insert(0) += 1;
                if *board.get(&idx).unwrap().get(&c1.0).unwrap() == 2 {
                    overlaps += 1;
                }
                idx += 1;
            }
        } else {
            let mut left = c1;
            let mut right = c2;
            if right.0 < left.0 {
                left = c2;
                right = c1;
            }
            let mut direction: i32 = 1;
            if right.1 < left.1 {
                direction = -1;
            }
            while left.0 < right.0 + 1 {
                *board
                    .entry(left.1)
                    .or_insert(HashMap::<usize, usize>::new())
                    .entry(left.0)
                    .or_insert(0) += 1;
                if *board.get(&left.1).unwrap().get(&left.0).unwrap() == 2 {
                    overlaps += 1;
                }
                left.0 += 1;
                left.1 += direction;
            }
        }
    }
    overlaps
}

#[cfg(test)]
mod tests {
    use crate::get_overlaps;
    use crate::get_overlaps_with_diagonal;
    #[test]
    fn returns_correct_number_of_points() {
        let overlap = get_overlaps("test-input.txt");
        assert_eq!(overlap, 5);
    }

    #[test]
    fn returns_overlaps_with_diagonal() {
        let overlap = get_overlaps_with_diagonal("test-input.txt");
        assert_eq!(overlap, 12);
    }
}
