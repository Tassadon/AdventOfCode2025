use std::fs;
use std::collections::HashSet;

pub fn part_1() {
    let content = fs::read_to_string("day2.txt").expect("Failed to read file");
    let ranges_vec: Vec<&str> = content.split(',').collect();
    println!("File content:\n {ranges_vec:?} ");
    let ranges_vec_tuple: Vec<(u64, u64)> = ranges_vec
        .iter()
        .map(|s| {
            let mut bounds = s
                .split('-')
                .map(|num_str| num_str.parse::<u64>().expect("Failed to parse number"));
            (bounds.next().unwrap(), bounds.next().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();
    let mut length: u64;
    let mut first_half: u64;
    let mut second_half: u64;
    let mut count: u64 = 0;
    for range in &ranges_vec_tuple {
        for num in range.0..=range.1 {
            length = num.ilog10() as u64 + 1;
            if length % 2 == 0 {
                first_half = num / (10u64.pow((length / 2).try_into().unwrap()));
                second_half = num % (10u64.pow((length / 2).try_into().unwrap()));
                if first_half == second_half {
                    count += num;
                }
            }
        }
    }
    println!("Range: {:?}", count);
}

pub fn part_2() {
    let content = fs::read_to_string("day2.txt").expect("Failed to read file");
    let ranges_vec: Vec<&str> = content.split(',').collect();
    println!("File content:\n {ranges_vec:?} ");
    let ranges_vec_tuple: Vec<(u64, u64)> = ranges_vec
        .iter()
        .map(|s| {
            let mut bounds = s
                .split('-')
                .map(|num_str| num_str.parse::<u64>().expect("Failed to parse number"));
            (bounds.next().unwrap(), bounds.next().unwrap())
        })
        .collect::<Vec<(u64, u64)>>();

    let mut count: u64 = 0;
    let mut num_set: HashSet<u64> = HashSet::new();

    for range in &ranges_vec_tuple {
        for num in range.0..=range.1 {
            let temp_string = num.to_string();
            let length = temp_string.len();

            for window_size in 1..=(length / 2) {
                num_set.clear();
                if length % window_size != 0 {
                    continue;
                }
                for i in 0..(length / window_size) {
                    let substring = &temp_string[i * window_size..i * window_size + window_size];
                    let substring_num = substring.parse::<u64>().unwrap();
                    num_set.insert(substring_num);
                }
                if num_set.len() == 1 {
                    //println!("Found number: {}", num);
                    count += num;
                    break;
                }
            }
        }
    }
    println!("Range: {:?}", count);
}