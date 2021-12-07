use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("PART 1: {:?}", simulate_fish("input.txt", 80));
    println!("PART 2: {:?}", simulate_fish("input.txt", 256));
}

fn simulate_fish(filename: &str, days: usize) -> usize {
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    let line = reader.lines().next().unwrap().unwrap();
    let fish = line
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // population will contain count of fish in each part of the cycle
    let mut population = [0; 7];
    let mut virgin_population = [0; 9];
    for f in fish {
        // should be able to do this at same time as parsing input data
        population[f] += 1;
    }
    for day in 0..days {
        let idx_to_spawn = day % population.len();
        let count_of_fish_to_spawn = population[idx_to_spawn];

        let virgin_idx_to_spawn = day % virgin_population.len();
        let count_of_virgin_fish_to_spawn = virgin_population[virgin_idx_to_spawn];

        virgin_population[virgin_idx_to_spawn] += count_of_fish_to_spawn;
        population[idx_to_spawn] += count_of_virgin_fish_to_spawn;
    }
    let mut sum = 0;
    for age_count in population {
        sum += age_count
    }
    for age_count in virgin_population {
        sum += age_count
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::simulate_fish;
    #[test]
    fn day_80() {
        assert_eq!(simulate_fish("test-input.txt", 80), 5934);
    }
    #[test]
    fn day_3() {
        assert_eq!(simulate_fish("test-input.txt", 3), 7);
    }
    #[test]
    fn day_18() {
        assert_eq!(simulate_fish("test-input.txt", 18), 26);
    }
}
