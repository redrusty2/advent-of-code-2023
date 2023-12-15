use std::{collections::BTreeMap, fs};

use indexmap::IndexMap;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();
    // let input = fs::read_to_string("input_example1.txt").unwrap();

    // let sum: usize = input
    //     .split(",")
    //     .map(|seq| {
    //         seq.chars()
    //             .filter(|c| c != &'\n')
    //             .map(|c| c as usize)
    //             .fold(0, |acc, c| ((acc + c) * 17) % 256)
    //     })
    //     .sum();

    // println!("Part one: {}", sum);
    //

    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::new());
    }

    for seq in input.split(",") {
        println!("================== seq {}", seq);
        let op_idx = match &seq[seq.len() - 1..seq.len()] {
            "-" => seq.len() - 1,
            _ => seq.len() - 2
        };
        let lens = &seq[0..op_idx];
        let op = &seq[op_idx..op_idx + 1];
        let box_idx = hash(lens);

        match op {
            "-" => {
                let pos = boxes[box_idx].iter().position(|(l, _)| l == &lens);
                if let Some(p) = pos {
                    boxes[box_idx].remove(p);
                }
            }
            "=" => {
                let pos = boxes[box_idx].iter().position(|(l, _)| l == &lens);
                let focal_len = seq[op_idx + 1..seq.len()].parse::<usize>().unwrap();
                if let Some(p) = pos {
                    boxes[box_idx][p] = (lens, focal_len);
                } else {
                    boxes[box_idx].push((lens, focal_len));
                }
            }
            _ => {
                panic!("Unknown op {} seq {}", op, seq);
            }
        }

        // for i in 0..4 {
        //     let bx = &boxes[i];
        //     println!("box {:?}", bx);
        // }
    }

    // for i in 0..4 {
    //     let bx = &boxes[i];
    //     println!("box {:?}", bx);
    // }
    for bx in boxes.iter() {
        println!("box {:?}", bx);
    }

    let sum: usize = boxes.iter().enumerate().map(|(box_idx, bx)| {
        bx.iter().enumerate().map(move |(slot, (_, focal_len))| {
            (box_idx + 1) * (slot + 1) * focal_len
        })
    }).flatten().sum();

    println!("Part 2: {}", sum);
}

fn hash(seq: &str) -> usize {
    seq.chars()
        .filter(|c| c != &'\n')
        .map(|c| c as usize)
        .fold(0, |acc, c| ((acc + c) * 17) % 256)
}
