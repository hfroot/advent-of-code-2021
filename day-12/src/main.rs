use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    part_1();
}

#[derive(Debug, PartialEq, Clone)]
enum CaveType {
    Big,
    Small,
    Start,
    End,
}

#[derive(Debug, PartialEq, Clone)]
struct Cave {
    feature: CaveType,
    name: String,
}

fn document_cave(caves: &mut HashMap<String, Cave>, name: &str) {
    let mut feature = CaveType::Small;
    if name.chars().last().unwrap().is_ascii_uppercase() {
        feature = CaveType::Big;
    } else if name == "start" {
        feature = CaveType::Start;
    } else if name == "end" {
        feature = CaveType::End;
    }
    caves.insert(name.to_string(), Cave { feature, name: name.to_string() });
}

struct Map {
    // cave -> connecting caves - should this be on Cave instead?
    cave_connections: HashMap<String, Vec<Cave>>,
    caves: HashMap<String, Cave>,
}

impl Map {
    fn paths(&self) -> Vec<Vec<Cave>> {
        let mut completed_paths = Vec::new();
        let start_cave = self.caves.get("start").unwrap().clone();
        let path = vec![start_cave];
        self.complete_path(&mut completed_paths, &path);
        completed_paths
    }

    fn complete_path(&self, completed_paths: &mut Vec::<Vec<Cave>>, path: &Vec::<Cave>) {
        let current_cave = path.iter().last().unwrap();
        if current_cave.feature == CaveType::End {
            completed_paths.push(path.to_vec());
        } else {
            let next_caves = self.next_caves(path);
            for nc in next_caves {
                let mut new_path = path.clone();
                new_path.push(nc);
                self.complete_path(completed_paths, &new_path);
            }
        }
    }

    fn next_caves(&self, path: &Vec::<Cave>) -> Vec<Cave> {
        let connections = self.cave_connections.get(&path.iter().last().unwrap().name).unwrap();
        let mut next_caves = Vec::<Cave>::new();
        let small_caves_in_path: Vec::<String> = path.iter().filter(|x| x.feature != CaveType::Big).map(|x| x.name.clone()).collect();
        let dead_ends = self.dead_ends();
        for cave in connections {
            if cave.feature == CaveType::Big {
                next_caves.push(cave.clone());
            } else if small_caves_in_path.iter().find(|&x| x == &cave.name).is_none() && dead_ends.get(&cave.name).is_none() {
                next_caves.push(cave.clone());
            }
        }
        next_caves
    }

    // if a cave is only connected to one small cave, that cave is a trap
    fn dead_ends(&self) -> HashSet<String> {
        let mut dead_ends = HashSet::<String>::new();
        for (cave_name, connections) in self.cave_connections.iter() {
            if connections.len() == 1 {
                if connections[0].feature != CaveType::Big {
                    dead_ends.insert(cave_name.to_string());
                }
            }
        }
        dead_ends
    }
}

fn new_map(filename: &str) -> Map {
    let mut caves = HashMap::<String, Cave>::new();
    let mut cave_connections = HashMap::<String, Vec<Cave>>::new();
    let file = File::open(filename).expect("File not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split("-").collect::<Vec<&str>>();
        let a = parts[0];
        let b = parts[1];
        if cave_connections.get(a).is_none() {
            document_cave(&mut caves, a);
            cave_connections.insert(a.clone().to_string(), Vec::new());
        }
        if cave_connections.get(b).is_none() {
            document_cave(&mut caves, b);
            cave_connections.insert(b.clone().to_string(), Vec::new());
        }
        cave_connections.get_mut(a).unwrap().push(caves.get(&b.to_string()).unwrap().clone());
        cave_connections.get_mut(b).unwrap().push(caves.get(&a.to_string()).unwrap().clone());
    }
    Map {
        caves,
        cave_connections,
    }
}

fn part_1() {
    let map = new_map("input.txt");
    println!("PART 1: {}", map.paths().len());
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_nano() {
        let map = new_map("nano-input.txt");
        assert_eq!(map.caves.len(), 6);
        assert!(map.cave_connections.get("start").is_some());
        assert!(map.cave_connections.get("end").is_some());
        assert_eq!(map.paths().len(), 10);
    }
    #[test]
    fn test_dead_ends() {
        let map = new_map("micro-input.txt");
        assert!(map.dead_ends().get(&"sa".to_string()).is_some());
    }
    #[test]
    fn test_next_caves() {
        let map = new_map("micro-input.txt");
        // start,dc,kj,HN,end
        let next = map.next_caves(&vec![map.caves.get("start").unwrap().clone(), map.caves.get("dc").unwrap().clone(), map.caves.get("kj").unwrap().clone()]);
        assert_eq!(next.len(), 1);
        assert_eq!(next[0], map.caves.get("HN").unwrap().clone());
    }
    #[test]
    fn test_micro() {
        let map = new_map("micro-input.txt");
        assert_eq!(map.paths().len(), 19);
    }
    #[test]
    fn test_input() {
        let map = new_map("test-input.txt");
        assert_eq!(map.paths().len(), 226);
    }
}