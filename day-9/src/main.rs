use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("PART 1: {}", calculate_sum(get_low_points(read_input("input.txt"))));
}

fn get_low_points(input: Vec<Vec<usize>>) -> Vec<usize> {
    let mut points = vec![];
    let input = pad_input(input);
    for (y, row) in input.iter().enumerate() {
        if y == 0 || y == input.len() - 1 {
            continue;
        }
        for (x, number) in row.iter().enumerate() {
            if x == 0 || x == row.len() - 1 {
                continue;
            }
            if number < &row[x-1] && number < &row[x+1] && number < &input[y-1][x] && number < &input[y+1][x] {
                points.push(*number);
            }
        }
    }
    points
}

fn calculate_sum(points: Vec<usize>) -> usize {
    points.iter().sum::<usize>() + points.len()
}

fn pad_input(input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut padded_input = input.clone();
    let row_size = padded_input[0].len();
    padded_input.insert(0,vec![9;row_size]);
    padded_input.push(vec![9;row_size]);
    for row in &mut padded_input {
        row.insert(0,9);
        row.push(9);
    }
    padded_input
}

fn read_input(filename: &str) -> Vec<Vec<usize>> {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let mut values = Vec::new();
    for line in reader.lines() {
        values.push(
            line
                .unwrap()
                .chars()
                .map(|x| x.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        );
    }
    values
}

#[cfg(test)]
mod tests {
    use crate::get_low_points;
    use crate::calculate_sum;
    use crate::pad_input;
    use crate::read_input;
    #[test]
    fn input_correctly_padded() {
        let input = vec![vec![1,8,7], vec![8,5,6], vec![7,6,0]];
        assert_eq!(pad_input(input), vec![vec![9,9,9,9,9], vec![9,1,8,7,9], vec![9,8,5,6,9], vec![9,7,6,0,9], vec![9,9,9,9,9]])
    }
    #[test]
    fn correct_points_found() {
        let input = vec![vec![1,8,7], vec![8,5,6], vec![7,6,0]];
        let points = get_low_points(input);
        assert_eq!(points, vec![1,5,0]);
    }
    #[test]
    fn correct_sum_calculated() {
        let points = vec![1,5,0];
        let sum = calculate_sum(points);
        assert_eq!(sum, 9);
    }
    #[test]
    fn input_file_read() {
        let input = read_input("test-input-mini.txt");
        assert_eq!(input, vec![vec![1,8,7], vec![8,5,6], vec![7,6,0]]);
    }
}