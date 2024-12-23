use std::fs;

fn main() {
    let input = parse_input("./day7/input");
    let output1 = part_1(&input);
    println!("Part 1: {}", output1);
}

#[derive(Debug)]
struct Line {
    target: u64,
    ops: Vec<u64>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Op {
    Mult,
    Add,
}

fn parse_input(path: &str) -> Vec<Line> {
    let data = fs::read_to_string(path).expect("File exists");
    let mut output = Vec::new();
    for line in data.lines() {
        let (target, ops) = line.split_once(":").unwrap();
        output.push(Line {
            target: target.parse().unwrap(),
            ops: ops.split_whitespace().map(|v| v.parse().unwrap()).collect(),
        })
    }
    output
}

fn generate_calculation(ops: Vec<Op>, line: &Line) -> u64 {
    assert!(ops.len() == line.ops.len() - 1);
    let mut nums_iter = line.ops.iter();
    let mut output = nums_iter.next().unwrap().to_owned();
    for (i, num) in nums_iter.enumerate() {
        match ops[i] {
            Op::Mult => output *= num,
            Op::Add => output += num,
        }
    }
    output
}

/// Fast permutation generation
fn generate_permutations(len: usize, mult_count: usize) -> Vec<Vec<Op>> {
    let mut results = Vec::new();
    let mut ops = Vec::new();

    for _ in 0..(len - mult_count) {
        ops.push(Op::Add);
    }
    for _ in 0..mult_count {
        ops.push(Op::Mult);
    }

    let mut visited = vec![false; len];
    let mut current = Vec::with_capacity(len);

    fn backtrack(
        ops: &Vec<Op>,
        visited: &mut Vec<bool>,
        current: &mut Vec<Op>,
        results: &mut Vec<Vec<Op>>,
    ) {
        if current.len() == ops.len() {
            results.push(current.clone());
            return;
        }

        for i in 0..ops.len() {
            if visited[i] {
                continue;
            }

            if i > 0 && ops[i] == ops[i - 1] && !visited[i - 1] {
                continue;
            }

            visited[i] = true;
            current.push(ops[i].clone());
            backtrack(ops, visited, current, results);
            current.pop();
            visited[i] = false;
        }
    }

    ops.sort_by_key(|op| match op {
        Op::Add => 0,
        Op::Mult => 1,
    });

    backtrack(&ops, &mut visited, &mut current, &mut results);

    results
}

fn part_1(input: &Vec<Line>) -> u64 {
    let result = input
        .iter()
        .map(|line| {
            let mut mult_count = 0;
            while mult_count < line.ops.len() {
                let ops_permutations = generate_permutations(line.ops.len() - 1, mult_count);
                for perms in ops_permutations {
                    let calculation = generate_calculation(perms, &line);
                    if line.target == calculation {
                        return line.target;
                    }
                }
                mult_count += 1;
            }
            return 0;
        })
        .sum();
    result
}
