use std::{collections::VecDeque, fs, iter::repeat, sync::mpsc::channel};

use threadpool::ThreadPool;

fn test(input: String) {
    let sum: usize = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            println!("line {}", i);
            let mut split = line.split_whitespace();
            let m = split.next().unwrap();
            let c = split.next().unwrap();
            let mask_1: Vec<char> = repeat(m)
                .take(5)
                .collect::<Vec<&str>>()
                .join("?")
                .chars()
                .collect();
            let clusters_1: Vec<usize> = repeat(c)
                .take(5)
                .collect::<Vec<&str>>()
                .join(",")
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();
            println!("==================== Start of new line > {}", line);
            println!("mask {}", mask_1.iter().collect::<String>());
            println!("cluster {:?}", clusters_1);
            let combs_1 = segment(0, &clusters_1, &mask_1, "");
            println!("combs {}", combs_1);
            // println!("combs {}", combs_2);
            // println!("total {}", total);
            // return total;
            combs_1
        })
        .sum();

    println!("sum {}", sum)
}

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    // let input = fs::read_to_string("input1.txt").unwrap();
    let input = fs::read_to_string("my_input.txt").unwrap();
    
    test(input);
    return;

    let pool = ThreadPool::new(15);

    let (tx, rx) = channel();

    let mut job_count = 0;
    input.lines().enumerate().for_each(|(i, line)| {
        println!("line {}", i);
        let mut split = line.split_whitespace();
        let m = split.next().unwrap();
        let c = split.next().unwrap();
        // let mask_1: Vec<char> = repeat(m)
        //     .take(1)
        //     .collect::<Vec<&str>>()
        //     .join("?")
        //     .chars()
        //     .collect();
        // let clusters_1: Vec<usize> = repeat(c)
        //     .take(1)
        //     .collect::<Vec<&str>>()
        //     .join(",")
        //     .split(",")
        //     .map(|n| n.parse().unwrap())
        //     .collect();
        // let mask_2: Vec<char> = repeat(m)
        //     .take(2)
        //     .collect::<Vec<&str>>()
        //     .join("?")
        //     .chars()
        //     .collect();
        // let clusters_2: Vec<usize> = repeat(c)
        //     .take(2)
        //     .collect::<Vec<&str>>()
        //     .join(",")
        //     .split(",")
        //     .map(|n| n.parse().unwrap())
        //     .collect();
        let mask_5: Vec<char> = repeat(m)
            .take(5)
            .collect::<Vec<&str>>()
            .join("?")
            .chars()
            .collect();
        let clusters_5: Vec<usize> = repeat(c)
            .take(5)
            .collect::<Vec<&str>>()
            .join(",")
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect();

        job_count += 1;

        let tx = tx.clone();
        pool.execute(move || {
            // println!("==================== Start of new line > {}", line);
            // println!("mask {}", mask_2.iter().collect::<String>());
            // println!("cluster {:?}", clusters_2);
            // let combs_1 = segment(0, &clusters_1, &mask_1, "");
            // let combs_2 = segment(0, &clusters_2, &mask_2, "");
            // println!("combs {}", combs_1);
            // println!("combs {}", combs_2);
            // let rem = combs_2 % combs_1;
            // if rem == 0 {
            //     let factor = combs_2 / combs_1;
            //     let total = combs_1 * usize::pow(factor, 4);
            //     // println!("total {}", total);
            //     // return total;
            //     tx.send(total).unwrap();
            // }
            // println!("rem {}", rem);

            let combs = segment(0, &clusters_5, &mask_5, "");
            // println!("combs {}", combs);
            tx.send(combs).unwrap();
        });
    });

    let mut received = 0;
    let mut sum = 0;
    while received < job_count {
        let result = rx.recv().unwrap();
        println!("Got: {}, received {}", result, received);
        received += 1;
        sum += result;
    }

    println!("sum {}", sum);
}

fn segment(i: usize, clusters: &Vec<usize>, mask: &[char], before: &str) -> usize {
    // println!("-------- Start of {}", i);
    if i >= clusters.len() {
        if !mask.contains(&'#') {
            // println!(
            //     "end  {}",
            //     before.to_string() + &mask.iter().collect::<String>()
            // );
            return 1;
        }
        return 0;
    }
    // println!("mask {:?}", mask);

    let size = clusters[i];
    // println!("size {}", size);

    let mut piece = repeat('#').take(size).collect::<VecDeque<char>>();
    if i < clusters.len() - 1 {
        piece.push_back('.');
    }

    let mut seg_total = 0;
    loop {
        // can this piece go here?
        let valid = is_valid_pos(mask, &piece);
        let next_before = before.to_string() + &piece.iter().collect::<String>();
        if valid {
            seg_total += segment(
                i + 1,
                clusters,
                &mask[piece.len()..mask.len()],
                &next_before,
            );
        }
        piece.push_front('.');
        if piece.len() > mask.len() {
            break;
        }
    }

    return seg_total;
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
