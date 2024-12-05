#![feature(ascii_char, ascii_char_variants)]

use std::{ascii::Char, fs};

fn main() {
    let input = read_input("./day4/input");
    let output1 = part_1(&input);
    let output2 = part_2(&input);
    println!("Part 1: {}", output1);
    println!("Part 2: {}", output2);
}

fn read_input(path: &str) -> Vec<Vec<Char>> {
//     let data = ".M.S......
// ..A..MSMS.
// .M.S.MAA..
// ..A.ASMSM.
// .M.S.M....
// ..........
// S.S.S.S.S.
// .A.A.A.A..
// M.M.M.M.M.
// ..........";
    let data = fs::read_to_string(path).expect("File exists");
    let mut output = Vec::new();
    data.lines()
        .for_each(|line| output.push(line.as_ascii().unwrap().to_vec()));
    output
}

const MAS: &'static [Char] = match "MAS".as_ascii() {
    Some(ascii) => ascii,
    None => panic!("Invalid ascii!"),
};

// Rotate matrix 90 degrees
fn rotate_90_clockwise(matrix: Vec<Vec<Char>>) -> Vec<Vec<Char>> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut rotated = vec![vec![Char::Space; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            rotated[j][rows - i - 1] = matrix[i][j];
        }
    }

    rotated
}

fn part_1(input: &Vec<Vec<Char>>) -> u32 {
    let mut output: u32 = 0;
    // Let's find the first X, and then see if it has MAS to right, diagonal or horizontal
    let mut input_rotation = input.to_owned();
    for _rotation in 0..4 {
        for y in 0..input.len() {
            for x in 0..input_rotation[y].len() {
                let c = input_rotation[y][x];
                if c == Char::CapitalX {
                    output += match_horizontal_front(&input_rotation, x, y);
                    output += match_diagonal_front(&input_rotation, x, y);
                }
            }
        }
        input_rotation = rotate_90_clockwise(input_rotation);
    }
    output as u32
}

fn part_2(input: &Vec<Vec<Char>>) -> u32 {
    let mut output: u32 = 0;
    // Let's find the first X, and then see if it has MAS to right, diagonal or horizontal
    let mut input_rotation = input.to_owned();
    for _rotation in 0..4 {
        for y in 0..input.len() {
            for x in 0..input_rotation[y].len() {
                let c = input_rotation[y][x];
                if c == Char::CapitalM {
                    output += find_mas(&input_rotation, x, y);
                }
            }
        }
        input_rotation = rotate_90_clockwise(input_rotation);
    }
    output as u32
}

fn find_mas(input: &Vec<Vec<Char>>, x: usize, y: usize) -> u32 {
    if y + 3 > input.len() || x + 3 > input[y].len() {
        return 0;
    }
    // Ugly hard coded shit
    if input[y][x + 2] == Char::CapitalS
        && input[y + 1][x + 1] == Char::CapitalA
        && input[y + 2][x] == Char::CapitalM
        && input[y + 2][x + 2] == Char::CapitalS
    {
        return 1;
    }

    return 0;
}

// Returns 1 if match, 0 otherwise
fn match_horizontal_front(input: &Vec<Vec<Char>>, x: usize, y: usize) -> u32 {
    // First, check that we have enough chars left
    if x + 4 > input[y].len() {
        return 0;
    }
    let horizontal_row = &input[y][x + 1..x + 4];
    if horizontal_row == MAS {
        return 1;
    } else {
        return 0;
    }
}

// Returns 1 if match, 0 otherwise
fn match_diagonal_front(input: &Vec<Vec<Char>>, x: usize, y: usize) -> u32 {
    // First, check that we have enough chars left
    if y + 4 > input.len() || x + 4 > input[y].len() {
        return 0;
    }
    let mut diagonal_row = Vec::new();
    for i in 1..4 {
        diagonal_row.push(input[y + i][x + i])
    }
    if diagonal_row == *MAS {
        return 1;
    }
    return 0;
}
