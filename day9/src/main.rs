#![feature(iter_map_windows)]

use std::{fs, ops::Range};

fn main() {
    let input = parse_input("./day9/sample_input");
    let output1 = part_1(&input);
    println!("Part 1: {}", output1);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    id: usize,
    block_size: u8,
    free_space: u8,
}

#[allow(dead_code)]
fn debug_print_nodes(nodes: &Vec<Node>) {
    for node in nodes {
        for _ in 0..node.block_size {
            print!("{}", node.id);
        }
        for _ in 0..node.free_space {
            print!(".");
        }
    }
    println!();
}

#[allow(dead_code)]
fn debug_print_blocks(blocks: &Vec<Option<usize>>) {
    for block in blocks {
        match block {
            Some(v) => print!("{}", v),
            None => print!("."),
        }
    }
    println!();
}

fn parse_input(path: &str) -> Vec<Node> {
    let data = fs::read_to_string(path).expect("File exists");
    let mut output: Vec<Node> = Vec::new();
    let mut id = 0;
    for window in data
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<u8>>()
        .chunks(2)
    {
        let l = window[0];
        let r = if window.len() == 2 { window[1] } else { 0 };

        output.push(Node {
            id,
            block_size: l,
            free_space: r,
        });
        id += 1;
    }
    output
}

fn move_to_first_free(
    blocks: &mut Vec<Option<usize>>,
    last_free_index: &mut usize,
    last_back_free_index: &mut usize,
) {
    let mut rblock: Option<usize> = None;

    for (i, block) in blocks
        .iter_mut()
        .rev()
        .skip(*last_back_free_index)
        .enumerate()
    {
        if rblock.is_none() {
            rblock = *block;
            *block = None;
            *last_back_free_index = i;
        } else {
            break;
        }
    }
    for (i, block) in blocks.iter_mut().skip(*last_free_index).enumerate() {
        if block.is_none() {
            *block = rblock;
            *last_free_index = i;
            break;
        }
    }
}

fn is_sorted(blocks: &Vec<Option<usize>>, last_sorted_index: &mut usize) -> bool {
    let mut found_empty = false;
    for (i, block) in blocks.iter().skip(*last_sorted_index).enumerate() {
        if block.is_none() {
            found_empty = true;
        }
        if found_empty && block.is_some() {
            *last_sorted_index = i;
            return false;
        }
    }
    true
}

fn part_1(nodes: &Vec<Node>) -> usize {
    let mut blocks: Vec<Option<usize>> = Vec::new();

    for node in nodes {
        blocks.extend(vec![Some(node.id); node.block_size as usize]);
        blocks.extend(vec![None; node.free_space as usize]);
    }

    let mut last_free_index = 0;
    let mut last_back_free_index = 0;
    let mut last_sorted_index = 0;
    loop {
        move_to_first_free(&mut blocks, &mut last_free_index, &mut last_back_free_index);
        if is_sorted(&blocks, &mut last_sorted_index) {
            break;
        }
    }

    blocks
        .iter()
        .enumerate()
        .filter(|&(_, block)| block.is_some())
        .map(|(pos, block)| pos * block.unwrap())
        .sum()
}
