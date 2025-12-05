use std::fs;

pub fn part1() {
    let content = fs::read_to_string("day3.txt").expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();
    //println!("File content:\n {lines:?} ");
    // Implement the logic for day 3 part 1 here
    let mut battery_charge: u64 = 0;

    let mut largest_first: u64;
    let mut largest_second: u64;
    let mut cur_num: u64;
    let mut length: usize;
    for bank in &lines {
        largest_first = 0;
        largest_second = 0;
        length = bank.len();
        for (pos, ch) in bank.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                cur_num = digit as u64;
                if cur_num > largest_first && pos != length - 1 {
                    largest_second = 0;
                    largest_first = cur_num;
                } else if cur_num > largest_second {
                    largest_second = cur_num;
                }
            } else {
                println!("Invalid character in input: {}", ch);
                continue;
            }
        }
        battery_charge += largest_first * 10 + largest_second;
    }

    println!("Battery charge: {:?}", battery_charge);
}

pub fn part2() {
    let content = fs::read_to_string("day3.txt").expect("Failed to read file");
    let lines: Vec<&str> = content.lines().collect();
    //println!("File content:\n {lines:?} ");
    // Implement the logic for day 3 part 1 here
    let mut battery_charge: u64 = 0;

    let mut stack: [u64; 12] = [0; 12];
    let mut cur_num: u64;
    let mut length: usize;
    for bank in &lines {
        stack = [0; 12];
        length = bank.len();
        for (pos, ch) in bank.chars().enumerate() {
            if let Some(digit) = ch.to_digit(10) {
                cur_num = digit as u64;
                // Find the insertion point without holding a mutable reference
                let mut insert_at = None;
                for (j, val) in stack.iter().enumerate() {
                    if cur_num > *val && pos <= (length - 1) - (11 - j) {
                        insert_at = Some(j);
                        break;
                    }
                }

                // Now modify the stack
                if let Some(j) = insert_at {
                    stack[j + 1..].fill(0);
                    stack[j] = cur_num;
                }
            } else {
                println!("Invalid character in input: {}", ch);
                continue;
            }
        }
        for (i, val) in stack.iter().enumerate() {
            battery_charge += val * 10u64.pow((11 - i) as u32);
        }

    }

    println!("Battery charge: {:?}", battery_charge);
}