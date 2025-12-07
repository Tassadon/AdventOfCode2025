use std::collections::HashSet;
use std::fs;

pub fn part1() {
    let content_intervals = fs::read_to_string("day5_intervals.txt").expect("za file!");
    let content_vals = fs::read_to_string("day5_nums.txt").expect("za file!");
    let ranges: Vec<(u64, u64)> = content_intervals
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return None;
            }
            let start = parts[0].trim().parse::<u64>().ok()?;
            let end = parts[1].trim().parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();
    let nums: Vec<u64> = content_vals
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid number in day5_nums.txt"))
        .collect();

    let mut counter = 0;

    for seed in nums {
        for (start, end) in &ranges {
            if seed >= *start && seed <= *end {
                counter += 1;
                break;
            }
        }
    }

    println!("{counter}");
}

pub fn part1_alt() {
    let content_intervals = fs::read_to_string("day5_intervals.txt").expect("za file!");
    let content_vals = fs::read_to_string("day5_nums.txt").expect("za file!");
    let ranges: Vec<(u64, u64)> = content_intervals
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return None;
            }
            let start = parts[0].trim().parse::<u64>().ok()?;
            let end = parts[1].trim().parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();
    let nums: Vec<u64> = content_vals
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Invalid number in day5_nums.txt"))
        .collect();

    let count = nums
        .iter()
        .filter(|seed| {
            ranges
                .iter()
                .any(|(start, end)| **seed >= *start && **seed <= *end)
        })
        .count();

    println!("{count}");
}

pub fn part2() {
    let content_intervals = fs::read_to_string("day5_intervals.txt").expect("za file!");
    let ranges: Vec<(u64, u64)> = content_intervals
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() != 2 {
                return None;
            }
            let start = parts[0].trim().parse::<u64>().ok()?;
            let end = parts[1].trim().parse::<u64>().ok()?;
            Some((start, end))
        })
        .collect();

    let mut intervals: Vec<_> = Vec::new();

    let mut counters = 0;

    let mut new_interval: (u64, u64);
    let mut remove_old: bool;

    for (start, end) in ranges {
        new_interval = (start, end);
        for (pos, (cur_interval_start, cur_interval_end)) in intervals.iter().enumerate() {
            if start >= *cur_interval_start
                && end >= *cur_interval_end
                && start <= *cur_interval_end
            {
                new_interval = (*cur_interval_start, end);
                intervals.remove(pos);
                break;
            } else if start <= *cur_interval_start
                && end <= *cur_interval_end
                && start <= *cur_interval_end
            {
                new_interval = (start, *cur_interval_end);
                intervals.remove(pos);
                break;
            } else if start <= *cur_interval_start && end >= *cur_interval_end {
                // remove old
                intervals.remove(pos);
                break;
            } else if start >= *cur_interval_start && end <= *cur_interval_end {
                new_interval = (*cur_interval_start, *cur_interval_end);
                intervals.remove(pos);
                break;
            }
        }
        intervals.push(new_interval);
    }

    intervals.sort_by(|a, b| a.0.cmp(&b.0));

    let mut combined_intervals: Vec<(u64, u64)> = Vec::new();

    for pos in 0..intervals.len() {
        let cur_interval = intervals.get(pos).unwrap();
        let top_interval: (u64, u64) = combined_intervals.pop().unwrap();

        for pos_2 in 0..combined_intervals.len() {}
    }

    let to_range: u64 = intervals.iter().map(|tubbie| tubbie.1 - tubbie.0).sum();

    let total: u64 = to_range + intervals.len() as u64;

    println!("{total}");
    //println!("{sorted_ranges:?}");
    println!("{intervals:?}");
}
