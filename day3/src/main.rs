use std::fs;

use regex::Regex;

fn main() {
    let input = parse_input("./day3/input");
    let output1 = part_1(&input);
    let output2 = part_2(&input);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

fn parse_input(path: &str) -> String {
    // "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string()
    let data = fs::read_to_string(path).expect("File exists");
    data
}

fn parse_mult(input: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((?<l>[0-9]{1,3}),(?<r>[0-9]{1,3})\)").unwrap();
    let mut output = 0;
    mul_re.captures_iter(input).for_each(|cap| {
        let left: u32 = cap.name("l").unwrap().as_str().parse().unwrap();
        let right: u32 = cap.name("r").unwrap().as_str().parse().unwrap();
        output += left * right;
    });
    output
}

fn part_1(input: &str) -> u32 {
    let mut output = 0;
    for line in input.lines() {
        output += parse_mult(line);
    }
    output
}

fn part_2(input: &str) -> u32 {
    let control_re = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut output = 0;

    for line in input.lines() {
        let mut last_pos = 0;

        // Calculate in segments
        for cap in control_re.find_iter(line) {
            let control = cap.as_str();
            let control_pos = cap.start();

            if enabled {
                output += parse_mult(&line[last_pos..control_pos]);
            }

            // Update enabled
            enabled = control == "do()";

            last_pos = cap.end();
        }

        // Finally calculate remainder
        if enabled {
            output += parse_mult(&line[last_pos..]);
        }
    }

    output
}
