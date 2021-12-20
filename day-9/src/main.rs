use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
mod utils;

fn main() {
    println!("PART 1: {}", calculate_sum(get_low_points(read_input("input.txt")).0));
    println!("PART 2: {}", part_2("input.txt"));
}

fn get_low_points(input: Vec<Vec<usize>>) -> (Vec<usize>, Vec<(usize, usize)>) {
    let mut numbers = vec![];
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
                numbers.push(*number);
                points.push((x,y));
            }
        }
    }
    (numbers, points)
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

fn basin_sizes(input: Vec<Vec<usize>>) -> Vec<usize> {
    let mut basin_sizes = Vec::new();
    let mut next_basin_idx = 0;
    let padded_input = pad_input(input.clone());
    let mut extended_input = extend_input(padded_input);
    let low_points = get_low_points(input).1;
    for point in low_points {
        let mut size = 0;
        assign_basin(&mut extended_input, point, next_basin_idx, &mut size);
        basin_sizes.push(size);
        next_basin_idx += 1;
    }
    basin_sizes
}

fn assign_basin(input: &mut Vec<Vec<(usize, Option<usize>)>>, point: (usize, usize), basin_id: usize, basin_size: &mut usize) {
    if input[point.1][point.0].0 == 9 {
        return; // shouldn't get here?
    }
    // Some(basin_id) could probably been just a boolean but cba to refactor
    input[point.1][point.0].1 = Some(basin_id);
    *basin_size += 1;

    let up = input[point.1 - 1][point.0];
    if up.0 != 9 && up.1.is_none() {
        assign_basin(input, (point.0, point.1 - 1), basin_id, basin_size);
    }
    let right = input[point.1][point.0 + 1];
    if right.0 != 9 && right.1.is_none() {
        assign_basin(input, (point.0 + 1, point.1), basin_id, basin_size);
    }
    let down = input[point.1 + 1][point.0];
    if down.0 != 9 && down.1.is_none() {
        assign_basin(input, (point.0, point.1 + 1), basin_id, basin_size);
    }
    let left = input[point.1][point.0 - 1];
    if left.0 != 9 && left.1.is_none() {
        assign_basin(input, (point.0 - 1, point.1), basin_id, basin_size);
    }
}

fn basin_size_for_point(input: &Vec<Vec<usize>>, point: (usize, usize)) -> usize {
    let mut size = 0;
    let padded_input = pad_input((*input).clone());
    let mut extended_input = extend_input(padded_input);
    assign_basin(&mut extended_input, point, 0, &mut size);
    size
}

fn extend_input(input: Vec<Vec<usize>>) -> Vec<Vec<(usize, Option<usize>)>> {
    let mut extended_input = Vec::new();
    for row in input {
        let mut new_row = Vec::new();
        for number in row {
            new_row.push((number, None));
        }
        extended_input.push(new_row);
    }
    extended_input
}

fn calculate_product(numbers: [usize;3]) -> usize {
    numbers.iter().product()
}

fn top_three(numbers: Vec<usize>) -> [usize;3] {
    let mut numbers = numbers.clone();
    utils::merge_sort(&mut numbers);
    let len = numbers.len();
    [numbers[len-1], numbers[len-2], numbers[len-3]]
}

fn part_2(filename: &str) -> usize {
    let input = read_input(filename);
    let sizes = basin_sizes(input);
    let top = top_three(sizes);
    calculate_product(top)
}

#[cfg(test)]
mod tests {
    use crate::get_low_points;
    use crate::calculate_sum;
    use crate::pad_input;
    use crate::read_input;
    use crate::basin_sizes;
    use crate::basin_size_for_point;
    use crate::calculate_product;
    use crate::extend_input;
    use crate::top_three;
    use crate::part_2;
    #[test]
    fn input_correctly_padded() {
        let input = vec![vec![1,8,7], vec![8,5,6], vec![7,6,0]];
        assert_eq!(pad_input(input), vec![vec![9,9,9,9,9], vec![9,1,8,7,9], vec![9,8,5,6,9], vec![9,7,6,0,9], vec![9,9,9,9,9]])
    }
    #[test]
    fn correct_points_found() {
        let input = vec![vec![1,8,7], vec![8,5,6], vec![7,6,0]];
        let points = get_low_points(input.clone()).0;
        assert_eq!(points, vec![1,5,0]);
        let coordinates = get_low_points(input).1;
        assert_eq!(coordinates, vec![(1,1), (2,2), (3,3)]);
    }
    #[test]
    fn correct_coordinates_found() {
        let input = read_input("test-input.txt");
        let coordinates = get_low_points(input).1;
        assert_eq!(coordinates, vec![(2,1), (10,1), (3,3), (7,5)]);
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
    #[test]
    fn find_basin_sizes() {
        let input = read_input("test-input.txt");
        assert_eq!(basin_sizes(input), vec![3,9,14,9]);
    }
    #[test]
    fn find_basin_size() {
        let input = read_input("test-input.txt");
        // because of the padding, all coordinates are 1-indexed
        assert_eq!(basin_size_for_point(&input, (1,1)), 3);
        assert_eq!(basin_size_for_point(&input, (10,1)), 9);
        assert_eq!(basin_size_for_point(&input, (3,3)), 14);
        assert_eq!(basin_size_for_point(&input, (7,5)), 9);
    }
    #[test]
    fn product_calculated_correctly() {
        assert_eq!(calculate_product([2,3,4]), 24);
    }
    #[test]
    fn find_top_three() {
        assert_eq!(top_three(vec![1,4,1,5,7,2,3,9]), [9,7,5]);
    }
    #[test]
    fn extend_number() {
        let input = vec![vec![1,8,7], vec![8,5,6], vec![7,6,0]];
        let extended_input = extend_input(input);
        assert_eq!(extended_input[0][0].0, 1);
        assert!(extended_input[0][0].1.is_none())
    }
    #[test]
    fn part_2_works() {
        assert_eq!(part_2("test-input.txt"), 1134);
    }
}