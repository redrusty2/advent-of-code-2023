use std::{collections::BTreeMap, fs};

use indexmap::IndexMap;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    // let input = fs::read_to_string("input_example1.txt").unwrap();

    let mut boxes: Vec<IndexMap<&str, usize>> = Vec::new();
    for _ in 0..256 {
        boxes.push(IndexMap::new());
    }

    for seq in input.split(",") {
        let op_idx = match &seq[seq.len() - 1..seq.len()] {
            "-" => seq.len() - 1,
            _ => seq.len() - 2,
        };
        let lens = &seq[0..op_idx];
        let op = &seq[op_idx..op_idx + 1];
        let box_idx = hash(lens);

        match op {
            "-" => {
                boxes[box_idx].shift_remove(lens);
            }
            "=" => {
                let focal_len = seq[op_idx + 1..seq.len()].parse::<usize>().unwrap();
                boxes[box_idx].insert(lens, focal_len);
            }
            _ => {
                panic!("Unknown op {} seq {}", op, seq);
            }
        }
    }

    let sum: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_idx, bx)| {
            bx.iter()
                .enumerate()
                .map(move |(slot, (_, focal_len))| (box_idx + 1) * (slot + 1) * focal_len)
        })
        .flatten()
        .sum();

    println!("Part 2: {}", sum);
}

fn hash(seq: &str) -> usize {
    seq.chars()
        .filter(|c| c != &'\n')
        .map(|c| c as usize)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}
