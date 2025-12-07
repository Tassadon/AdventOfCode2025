use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

fn project_tachyon(
    tachyon_map: &Vec<Vec<char>>,
    tree_level: usize,
    pos: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    //println!()
    if let Some(next_level) = tachyon_map.get(tree_level) {
        if let Some(&next_char) = next_level.get(pos) {
            if visited.contains(&(tree_level, pos)) {
                return 0;
            } else {
                visited.insert((tree_level, pos));
            }

            if next_char == '.' || next_char == 'S' {
                return project_tachyon(tachyon_map, tree_level + 1, pos, visited);
            } else if next_char == '^' {
                return 1
                    + project_tachyon(tachyon_map, tree_level, pos - 1, visited)
                    + project_tachyon(tachyon_map, tree_level, pos + 1, visited);
            } else {
                return 0;
            }
        } else {
            return 0;
        }
    } else {
        return 0;
    }
}

pub fn part1() {
    let contents = fs::read_to_string("day7.txt").expect("A FILE!");

    let content_fungus: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    println!("{content_fungus:?}");

    let starting_pos = content_fungus
        .get(0)
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();

    println!("{starting_pos}");

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let splits = project_tachyon(&content_fungus, 0, starting_pos, &mut visited);

    println!("length of total shit: {}", content_fungus.len());

    println!("number of splits: {}", splits);
}

pub fn part2() {
    let contents = fs::read_to_string("day7.txt").expect("A FILE!");

    let content_fungus: Vec<Vec<char>> = contents.lines().map(|s| s.chars().collect()).collect();

    let splitters: Vec<_> = contents
        .lines()
        .into_iter()
        .flat_map(|line| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(move |(col_idx, _)| col_idx)
        })
        .collect();

    let starting_pos = content_fungus
        .get(0)
        .unwrap()
        .iter()
        .position(|&c| c == 'S')
        .unwrap();

    println!("{starting_pos}");

    let mut counters: HashMap<usize,usize> = HashMap::new();
    
    counters.insert(starting_pos,1);

    let mut my_fungus: bool = false;

    for splitter in splitters{

        if let Some(&val) = counters.get(&splitter){

            if let Some(&val_1) = counters.get(&(splitter+1)){
                counters.insert(splitter+1,val+val_1);
            }else{
                counters.insert(splitter+1,val);
            }

            if let Some(&val_2) = counters.get(&(splitter-1)){
                counters.insert(splitter-1,val_2+val);
            }else{
                counters.insert(splitter-1,val);
            }

            my_fungus = true;
        }

        if my_fungus{
            counters.remove(&splitter);
            my_fungus = false;
        }

    }

    println!("The funguses are {}", counters.values().sum::<usize>());

}