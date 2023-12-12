use std::{collections::VecDeque, fs, iter::repeat};

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();
    // let input = fs::read_to_string("my_input.txt").unwrap();

    let sum: usize = input
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            // split by known clusters
            let mask: Vec<char> = split.next().unwrap().chars().collect();
            let clusters: Vec<usize> = split
                .next()
                .unwrap()
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();

            println!("==================== Start of new line > {}", line);
            println!("mask {}", mask.iter().collect::<String>());
            let mut sums = Vec::new();
            segment(0, &clusters, &mask, &mut sums, "");
            println!("sums {:?}", sums);
            sums.last().unwrap().clone()
        })
        .sum();

    println!("Part one: {sum}");
    //
    // calc possibilities in each segment
    //
    // multiply possibilities together
}

fn segment(
    i: usize,
    clusters: &Vec<usize>,
    mask: &[char],
    sums: &mut Vec<usize>,
    before: &str,
) -> bool {
    // println!("-------- Start of {}", i);
    if i >= clusters.len() {
        if !mask.contains(&'#') {
            // println!(
            //     "end  {}",
            //     before.to_string() + &mask.iter().collect::<String>()
            // );
            return true;
        }
        return false;
    }
    // println!("mask {:?}", mask);

    if sums.get(i).is_none() {
        sums.push(0);
    }

    let size = clusters[i];
    // println!("size {}", size);

    let mut piece = repeat('#').take(size).collect::<VecDeque<char>>();
    if i < clusters.len() - 1 {
        piece.push_back('.');
    }

    let mut one_was_valid = false;
    loop {
        // can this piece go here?
        let valid = is_valid_pos(mask, &piece);
        let next_before = before.to_string() + &piece.iter().collect::<String>();
        if valid {
            let seg_valid = segment(
                i + 1,
                clusters,
                &mask[piece.len()..mask.len()],
                sums,
                &next_before,
            );

            if seg_valid {
                one_was_valid = true;
                sums[i] += 1;
            }
        } else {
        }

        piece.push_front('.');
        if piece.len() > mask.len() {
            break;
        }
    }

    return one_was_valid;
}

fn is_valid_pos(mask: &[char], piece: &VecDeque<char>) -> bool {
    for (i, p) in piece.iter().enumerate() {
        if i >= mask.len() {
            return false;
        }
        let m = mask[i];
        if m != '?' && m != *p {
            return false;
        }
    }
    true
}
