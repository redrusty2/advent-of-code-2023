use std::{fs, str::Lines};

#[derive(Debug)]
struct Pair {
    from: i64,
    to: i64,
    diff: i64,
}

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    part_one(input.clone());
    part_two(input);
}

fn part_two(input: String) {
    let mut lines = input.lines();

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    lines.next();

    let mut maps = Vec::new();
    while let Some(_) = lines.next() {
        maps.push(create_map(&mut lines));
    }

    let smallest_location = seeds
        .chunks(2)
        .inspect(|w| println!("{} {}", w[0], w[1]))
        .flat_map(|chunk| {
            (chunk[0]..chunk[0] + chunk[1])
                .into_iter()
                .map(|seed| maps.iter().fold(seed, |acc, m| convert(acc, m)))
        })
        .min()
        .unwrap();

    println!("Part two: {}", smallest_location);
}

fn part_one(input: String) {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap());

    lines.next();

    let mut maps = Vec::new();
    while let Some(_) = lines.next() {
        maps.push(create_map(&mut lines));
    }

    let smallest_location = seeds
        .map(|seed| maps.iter().fold(seed, |acc, m| convert(acc, &m)))
        .min()
        .unwrap();

    println!("Part one: {}", smallest_location);
}

fn convert(source: i64, map: &Vec<Pair>) -> i64 {
    for e in map {
        if source >= e.from && source < e.to {
            return e.diff + source;
        }
    }
    source
}

fn create_map(lines: &mut Lines) -> Vec<Pair> {
    let mut v = Vec::new();
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }

        let mut ns = l.split_whitespace();
        let dest_start: i64 = ns.next().unwrap().parse().unwrap();
        let source_start: i64 = ns.next().unwrap().parse().unwrap();
        let length: i64 = ns.next().unwrap().parse().unwrap();

        v.push(Pair {
            from: source_start,
            to: source_start + length,
            diff: dest_start - source_start,
        });
    }
    v.sort_by(|a, b| {
        return a.from.cmp(&b.from);
    });
    v
}
