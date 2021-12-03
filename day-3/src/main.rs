use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let mut log: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();
        // couldn't understand why using Char instead of Vec<char>
        // resulted in error EO597 'text doesn't live long enough'
        let v: Vec<char> = text.chars().collect();
        log.push(v);
    }

    part1(log);
}

fn part1(log: Vec<Vec<char>>) {
    let mut sums = Vec::<i32>::new();
    let threshold = log.len() as i32 / 2;
    for binary_num in log {
        for idx in 0..binary_num.len() {
            if sums.len() < binary_num.len() {
                sums.push(0);
            }
            if binary_num[idx] == '1' {
                sums[idx] += 1;
            }
        }
    }
    let mut power = sums.len() as u32;
    let mut g_res: i32 = 0;
    let mut e_res: i32 = 0;
    for sum in sums {
        power -= 1;
        if sum >= threshold {
            g_res += 2_i32.pow(power);
        } else {
            e_res += 2_i32.pow(power);
        }
    }
    println!("{:?}", g_res * e_res);
}