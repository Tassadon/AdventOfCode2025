use std::collections::HashSet;
use std::fs;

pub fn part_1(file_path: String) {
    let contents: String =
        fs::read_to_string(file_path).expect("Should ahve been able to read the file");
    let mut hash_fungus: HashSet<(i32, i32)> = HashSet::new();

    let vec_contents: Vec<_> = contents.split('\n').collect();
    let height = vec_contents.len();
    let width: usize = vec_contents.get(0).unwrap().len();

    for (col, line) in vec_contents.iter().enumerate() {
        for (row, character) in line.chars().enumerate() {
            if character == '@' {
                hash_fungus.insert((row as i32, col as i32));
            }
        }
    }

    let mut neighbors: i32;
    let mut counter: i32 = 0;

    for col in 0..=height {
        for row in 0..=width {
            if hash_fungus.contains(&(row as i32, col as i32)) {
                neighbors = 0;
                for leftes in -1..=1 {
                    for uppies in -1..=1 {
                        if leftes == 0 && uppies == 0 {
                            continue;
                        }

                        if hash_fungus.contains(&(row as i32 + leftes, col as i32 + uppies)) {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors < 4 {
                    counter += 1;
                }
            }
        }
    }

    println!("{counter}");
}

pub fn part_2(file_path: String) {
    let contents: String =
        fs::read_to_string(file_path).expect("Should ahve been able to read the file");
    let mut hash_fungus: HashSet<(i32, i32)> = HashSet::new();

    let vec_contents: Vec<_> = contents.split('\n').collect();
    let height = vec_contents.len();
    let width: usize = vec_contents.get(0).unwrap().len();

    for (col, line) in vec_contents.iter().enumerate() {
        for (row, character) in line.chars().enumerate() {
            if character == '@' {
                hash_fungus.insert((row as i32, col as i32));
            }
        }
    }

    let mut neighbors: i32;
    let mut counter: i32 = 0;
    let mut prev_counter: i32 = -1;

    while counter - prev_counter != 0 {
        prev_counter = counter;
        for col in 0..=height {
            for row in 0..=width {
                if hash_fungus.contains(&(row as i32, col as i32)) {
                    neighbors = 0;
                    for leftes in -1..=1 {
                        for uppies in -1..=1 {
                            if leftes == 0 && uppies == 0 {
                                continue;
                            }

                            if hash_fungus.contains(&(row as i32 + leftes, col as i32 + uppies)) {
                                neighbors += 1;
                            }
                        }
                    }
                    if neighbors < 4 {
                        counter += 1;
                        hash_fungus.remove(&(row as i32, col as i32));
                    }
                }
            }
        }
    }
    println!("{counter}");
}
