#![feature(iter_map_windows)]

use std::fs;

fn main() {
    let input = read_input("./day2/input");
    let output1 = part_1(&input);
    let output2 = part_2(&input);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

fn read_input(path: &str) -> Vec<Vec<i32>> {
    // vec![
    //     vec![7, 6, 4, 2, 1],
    //     vec![1, 2, 7, 8, 9],
    //     vec![9, 7, 6, 2, 1],
    //     vec![1, 3, 2, 4, 5],
    //     vec![8, 6, 4, 4, 1],
    //     vec![1, 3, 6, 7, 9],
    // ]
    let data = fs::read_to_string(path).expect("File exists");
    let mut output: Vec<Vec<i32>> = Vec::with_capacity(data.lines().count());
    for line in data.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        output.push(row);
    }
    output
}

fn part_1(input: &Vec<Vec<i32>>) -> i32 {
    let mut output = 0;
    for row in input {
        if row_safe(row) {
            output += 1;
        }
    }
    output
}

fn row_safe(input: &Vec<i32>) -> bool {
    let diffs = input.iter().map_windows(|&[l, r]| l - r);

    // Check if all numbers are increasing or decreasing
    let all_negative = diffs.clone().all(|v| v.is_negative());
    let all_positive = diffs.clone().all(|v| v.is_positive());
    if !all_positive && !all_negative {
        return false;
    }

    // Check if all are in distance of 1 - 3 of each other
    let all_in_distance = diffs.map(|v| v.abs()).all(|v| 1 <= v && v <= 3);
    if !all_in_distance {
        return false;
    }
    true
}

// Really stupid way, idk if there are any better
fn part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut output = 0;
    let mut row_with_removed: Vec<i32>;
    for row in input {
        // Default case
        if row_safe(row) {
            output += 1;
            continue;
        }
        // Check all variants
        for i in 0..row.len() {
            row_with_removed = row.clone();
            row_with_removed.remove(i);
            if row_safe(&row_with_removed) {
                output += 1;
                break;
            }
        }
    }
    output
}
