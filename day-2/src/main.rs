use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found!");
    let reader = BufReader::new(file);
    let mut log: Vec<(i32, char)> = [].to_vec();

    for line in reader.lines() {
        let text = line.unwrap();
        let v: Vec<&str> = text.split(' ').collect();
        let direction = v[0].chars().nth(0).unwrap();
        let distance = v[1].parse::<i32>().unwrap();
        log.push((distance, direction));
    }

    // would be nice to figure out how to not have to clone the log
    part1(log.clone());
    part2(log);
}

fn part1(log: Vec<(i32, char)>) {
    let mut depth = 0;
    let mut x = 0;
    for (distance, direction) in log {
        if direction == 'd' {
            depth += distance;
        } else if direction == 'u' {
            depth -= distance;
        } else if direction == 'f' {
            x += distance;
        }
    }
    println!("result {}", x * depth);
}

fn part2(log: Vec<(i32, char)>) {
    let mut depth = 0;
    let mut x = 0;
    let mut aim = 0;
    for (units, direction) in log {
        if direction == 'd' {
            aim += units;
        } else if direction == 'u' {
            aim -= units;
        } else if direction == 'f' {
            x += units;
            depth += aim * units;
        }
    }
    println!("result {}", x * depth);
}