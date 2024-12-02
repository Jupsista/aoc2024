use std::{fs, iter::zip};

fn main() {
    let (mut input_l, mut input_r) = read_input("./day1/input");
    input_l.sort();
    input_r.sort();

    let output1 = part_1(&input_l, &input_r);
    let output2 = part_2(&input_l, &input_r);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

fn read_input(path: &str) -> (Vec<u32>, Vec<u32>) {
    // (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])
    let data = fs::read_to_string(path).expect("File exists");
    let mut output_l: Vec<u32> = Vec::with_capacity(data.lines().count());
    let mut output_r: Vec<u32> = Vec::with_capacity(data.lines().count());
    for line in data.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        output_l.push(split[0].parse().unwrap());
        output_r.push(split[1].parse().unwrap());
    }
    (output_l, output_r)
}

fn part_1(input_l: &Vec<u32>, input_r: &Vec<u32>) -> u32 {
    let mut output = 0;
    for (l, r) in zip(input_l, input_r) {
        if r > l {
            output += r - l;
        } else {
            output += l - r;
        }
    }
    output
}

fn part_2(input_l: &Vec<u32>, input_r: &Vec<u32>) -> u32 {
    let mut output = 0;
    for l in input_l {
        let mut count = 0;
        // NOTE: This is inefficient since this goes over the list every time from the start
        for r in input_r {
            if l == r {
                count += 1;
            }
        }
        output += l * count;
    }
    output
}
