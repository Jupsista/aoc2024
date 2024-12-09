use std::fs;

fn main() {
    let input = parse_input("./day6/input");
    let mut positions = Vec::new();
    let output1 = part_1(&input, &mut positions);
    println!("Part 1: {}", output1);
}

fn parse_input(path: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(path).expect("File exists");
    let mut output = Vec::new();
    for line in data.lines() {
        output.push(line.chars().collect());
    }
    output
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn find_char_index(input: &Vec<Vec<char>>) -> (usize, usize, Direction) {
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            match *c {
                '^' => return (x, y, Direction::Up),
                '>' => return (x, y, Direction::Right),
                'v' => return (x, y, Direction::Down),
                '<' => return (x, y, Direction::Left),
                _ => (),
            }
        }
    }
    panic!("Should always find a starting index!");
}

fn find_next_position(
    input: &Vec<Vec<char>>,
    direction: &Direction,
    starting_pos: (usize, usize),
) -> Option<(usize, usize)> {
    let (start_x, start_y) = starting_pos;

    match direction {
        Direction::Up => (0..start_y)
            .rev()
            .find(|&y| input[y][start_x] == '#')
            .map(|y| (start_x, y + 1)),

        Direction::Right => (start_x + 1..input[start_y].len())
            .find(|&x| input[start_y][x] == '#')
            .map(|x| (x - 1, start_y)),

        Direction::Down => (start_y + 1..input.len())
            .find(|&y| input[y][start_x] == '#')
            .map(|y| (start_x, y - 1)),

        Direction::Left => (0..start_x)
            .rev()
            .find(|&x| input[start_y][x] == '#')
            .map(|x| (x + 1, start_y)),
    }
}

fn mark_path_visited(
    input: &mut Vec<Vec<char>>,
    starting_pos: (usize, usize),
    next_pos: (usize, usize),
    direction: &Direction,
) {
    let (start_x, start_y) = starting_pos;
    let (next_x, next_y) = next_pos;
    match direction {
        Direction::Up => {
            for y in (next_y..=start_y).rev() {
                input[y][start_x] = 'X';
            }
            input[next_y][start_x] = '>';
        }
        Direction::Right => {
            for x in start_x..=next_x {
                input[start_y][x] = 'X';
            }
            input[start_y][next_x] = 'v';
        }
        Direction::Down => {
            for y in start_y..=next_y {
                input[y][start_x] = 'X';
            }
            input[next_y][start_x] = '<';
        }
        Direction::Left => {
            for x in (next_x..=start_x).rev() {
                input[start_y][x] = 'X';
            }
            input[start_y][next_x] = '^';
        }
    }
}
/// Count visited parts, where the last position is accounted for
fn count_visited(input: &Vec<Vec<char>>) -> usize {
    input
        .iter()
        .flatten()
        .filter(|c| **c == 'X' || **c == '^' || **c == '>' || **c == 'v' || **c == '<')
        .count()
}

fn part_1(raw_input: &Vec<Vec<char>>, positions: &mut Vec<((usize, usize), Direction)>) -> usize {
    // Find the start
    let mut input = raw_input.clone();
    loop {
        let (start_x, start_y, dir) = find_char_index(&input);
        if let Some(next_pos) = find_next_position(&input, &dir, (start_x, start_y)) {
            mark_path_visited(&mut input, (start_x, start_y), next_pos, &dir);
            positions.push(((start_x, start_y), dir));
        } else {
            let input_height = input.len();
            let input_width = input[0].len();
            let mut final_fn = |next_x, next_y| {
                mark_path_visited(&mut input, (start_x, start_y), (next_x, next_y), &dir)
            };

            // Mark final path!
            match dir {
                Direction::Up => final_fn(start_x, 0),
                Direction::Right => final_fn(input_width - 1, start_y),
                Direction::Down => final_fn(start_x, input_height - 1),
                Direction::Left => final_fn(0, start_y),
            }
            break;
        }
    }
    count_visited(&input)
}
