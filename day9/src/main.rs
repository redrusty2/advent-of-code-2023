use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    part_two(input);
}

fn part_two(input: String) {
    let predictions: i64 = input
        .lines()
        .map(|line| {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            let mut levels: Vec<Vec<i64>> = vec![values.clone()];
            let mut i = 0;
            loop {
                let mut nexts = Vec::new();
                let mut currs = levels.get_mut(i).unwrap().iter();
                let mut prev: i64 = *currs.next().unwrap();
                let mut all_zero = true;

                for c in currs {
                    let diff = c - prev;
                    nexts.push(diff);
                    if diff != 0 {
                        all_zero = false;
                    }
                    prev = *c;
                }

                println!("next {:?}", nexts);

                if all_zero {
                    break;
                }

                levels.push(nexts);
                i += 1;
            }

            let mut last_diff = levels.last().unwrap().first().unwrap().clone();
            // levels.last_mut().unwrap().push(last_diff);

            for  l in levels.iter().rev().skip(1) {
                let next_last = l.first().unwrap();
                last_diff = next_last - last_diff;
            }

            last_diff
        })
            .sum();

    println!("Part one: {:?}", predictions);
}

fn part_one(input: String) {
    let predictions: i64 = input
        .lines()
        .map(|line| {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();

            let mut levels: Vec<Vec<i64>> = vec![values.clone()];
            let mut i = 0;
            loop {
                let mut nexts = Vec::new();
                let mut currs = levels.get_mut(i).unwrap().iter();
                let mut prev: i64 = *currs.next().unwrap();
                let mut all_zero = true;

                for c in currs {
                    let diff = c - prev;
                    nexts.push(diff);
                    if diff != 0 {
                        all_zero = false;
                    }
                    prev = *c;
                }

                println!("next {:?}", nexts);

                if all_zero {
                    break;
                }

                levels.push(nexts);
                i += 1;
            }

            let mut last_diff = levels.last().unwrap().last().unwrap().clone();
            // levels.last_mut().unwrap().push(last_diff);

            for  l in levels.iter().rev().skip(1) {
                let next_last = l.last().unwrap();
                last_diff += next_last;
            }

            last_diff
        })
            .sum();

    println!("Part one: {:?}", predictions);
}
