use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    // should make this a util
    let file = File::open("input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let values = line
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("PART 1: {}", min_fuel(&mut values.clone()));
    println!("PART 2: {}", min_fuel_2(values));
}

fn merge_sort(array: &mut Vec<usize>) {
    let mut helper = vec![0; array.len()];
    merge_sort_helper(array, &mut helper, 0, array.len() - 1);
}

fn merge_sort_helper(array: &mut Vec<usize>, helper: &mut Vec<usize>, low: usize, high: usize) {
    if low < high {
        let middle = (low + high) / 2;
        merge_sort_helper(array, helper, low, middle);
        merge_sort_helper(array, helper, middle + 1, high);
        merge(array, helper, low, middle, high);
    }
}

fn merge(array: &mut Vec<usize>, helper: &mut Vec<usize>, low: usize, middle: usize, high: usize) {
    for i in low..high + 1 {
        helper[i] = array[i];
    }
    let mut helper_left = low;
    let mut helper_right = middle + 1;
    let mut current = low;

    while helper_left <= middle && helper_right <= high {
        if helper[helper_left] <= helper[helper_right] {
            array[current] = helper[helper_left];
            helper_left += 1;
        } else {
            array[current] = helper[helper_right];
            helper_right += 1;
        }
        current += 1;
    }

    if helper_left <= middle {
        let remaining = middle - helper_left;
        for i in 0..remaining + 1 {
            array[current + i] = helper[helper_left + i];
        }
    }
}

fn median(array: &mut Vec<usize>) -> usize {
    merge_sort(array);
    if array.len() % 2 != 0 {
        array[array.len() / 2]
    } else {
        let v1 = array[array.len() / 2 - 1];
        let v2 = array[array.len() / 2];
        (v1 + v2) / 2
    }
}

fn min_fuel(array: &mut Vec<usize>) -> usize {
    let med = median(array);
    let mut fuel = 0;
    for v in array {
        if *v < med {
            fuel += med - *v;
        } else {
            fuel += *v - med;
        }
    }
    fuel
}

fn round(number: f64) -> usize {
    let rounded_down = number as usize;
    if (number * 10 as f64) as usize - rounded_down > 5 {
        rounded_down + 1
    } else {
        rounded_down
    }
}

fn min_fuel_2(array: Vec<usize>) -> usize {
    let mut sum = 0;
    for a in &array {
        sum += a;
    }
    let avg = round(sum as f64 / array.len() as f64);
    let mut fuel = 0;
    for v in array {
        let steps: usize;
        if v < avg {
            steps = avg - v;
        } else {
            steps = v - avg;
        }
        fuel += (steps * (steps + 1)) / 2;
    }
    fuel
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn merge_sort_works() {
        let mut array = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        merge_sort(&mut array);
        assert_eq!(array, [0, 1, 1, 2, 2, 2, 4, 7, 14, 16]);
    }

    #[test]
    fn merge_works_tiny() {
        let mut array = vec![1, 3, 2];
        merge(&mut array, &mut vec![0, 0, 0], 0, 1, 2);
        assert_eq!(array, [1, 2, 3]);
    }

    #[test]
    fn find_median_works() {
        let mut array = vec![0, 9, 56, 2, 3, 2, 2, 6, 0];
        assert_eq!(median(&mut array), 2);
    }

    #[test]
    fn find_median_works_even() {
        let mut array = vec![0, 9, 56, 2, 5, 2, 2, 6];
        assert_eq!(median(&mut array), 3);
    }

    #[test]
    fn find_min_fuel() {
        let mut array = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(min_fuel(&mut array), 37);
    }

    #[test]
    fn find_min_fuel_2() {
        let array = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(min_fuel_2(array), 168);
    }
}
