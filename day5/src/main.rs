#![feature(iter_map_windows)]

use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let (map, vec) = parse_input("./day5/input");
    let output1 = part_1(&map, &vec);
    let output2 = part_2(&map, &vec);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

fn parse_input(path: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let data = fs::read_to_string(path).expect("File exists");
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates = Vec::new();
    for line in data.lines() {
        if line.contains("|") {
            line.split("|")
                .map_windows(|[l, r]| (l.parse().unwrap(), r.parse().unwrap()))
                .for_each(|(l, r)| {
                    if rules.contains_key(&l) {
                        rules.get_mut(&l).unwrap().push(r);
                    } else {
                        rules.insert(l, vec![r]);
                    }
                });
        } else if !line.is_empty() {
            let vals = line.split(",").map(|v| v.parse().unwrap()).collect();
            updates.push(vals)
        }
    }
    (rules, updates)
}

// Check if a line is correct, by storing the previously
// encountered numbers and testing against them
fn is_line_correct(map: &HashMap<u32, Vec<u32>>, line: &Vec<u32>) -> bool {
    let mut encountered: HashSet<u32> = HashSet::new();
    for num in line {
        match map.get(&num) {
            Some(val) => match val.iter().find(|v| encountered.contains(v)) {
                Some(_) => {
                    return false;
                }
                None => (),
            },
            None => (),
        }
        encountered.insert(*num);
    }
    true
}

fn part_1(map: &HashMap<u32, Vec<u32>>, lines: &Vec<Vec<u32>>) -> u32 {
    let mut output = 0;
    for line in lines {
        if is_line_correct(map, line) {
            output += line[line.len() / 2];
        }
    }
    output
}

// Part 2, if number in line is in wrong order, swaps it with the first instance
// of a number that should be before it
fn part_2(map: &HashMap<u32, Vec<u32>>, lines: &Vec<Vec<u32>>) -> u32 {
    let mut encountered: HashSet<u32> = HashSet::new();
    let mut output = 0;
    let mut correct_line;
    let mut number_wrong_index;
    let mut number_swap_index;
    for line in lines {
        if !is_line_correct(map, line) {
            correct_line = line.clone();
            number_wrong_index = None;
            number_swap_index = None;
            'outer: loop {
                encountered.clear();
                if let (Some(wrong_index), Some(swap_index)) =
                    (number_wrong_index, number_swap_index)
                {
                    correct_line.swap(wrong_index, swap_index);
                }
                for (i, num) in correct_line.iter().enumerate() {
                    match map.get(&num) {
                        Some(val) => match val.iter().find(|v| encountered.contains(v)) {
                            Some(wrong) => {
                                number_wrong_index = Some(i);
                                // Find the index of the wrong number
                                number_swap_index =
                                    Some(correct_line.iter().position(|v| v == wrong).unwrap());
                                continue 'outer;
                            }
                            None => (),
                        },
                        None => (),
                    }
                    encountered.insert(*num);
                }
                break;
            }
            output += correct_line[correct_line.len() / 2];
        }
    }
    output
}
