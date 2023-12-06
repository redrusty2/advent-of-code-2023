use std::{fs, str::Lines};

#[derive(Debug)]
struct Pair {
    from: i64,
    to: i64,
    diff: i64,
}

#[derive(Debug)]
struct Range(i64, i64);

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    part_one(input.clone());
    part_two(input);
}

fn part_two(input: String) {
    let mut lines = input.lines();

    let split: Vec<i64> = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut seeds: Vec<_> = split.chunks(2).collect();

    seeds.sort_by(|a, b| {
        return a[0].cmp(&b[0]);
    });

    lines.next();

    let mut maps = Vec::new();
    while let Some(_) = lines.next() {
        maps.push(create_map(&mut lines));
    }

    let smallest_location = seeds
        .iter()
        .map(|chunk| Range(chunk[0], chunk[1]))
        .inspect(|r| println!("input {:?}", r))
        .flat_map(|range| maps.iter().fold(vec![range], |acc, m| convert_range(&acc, m)))
        .map(|range| range.0)
        .min().unwrap();

    println!("Part two: {}", smallest_location);
}

fn convert_range(source_ranges: &Vec<Range>, map: &Vec<Pair>) -> Vec<Range> {
    let mut ranges: Vec<Range> = Vec::new();

    for source in source_ranges {
        let mut curr = source.0;
        let source_end = source.0 + source.1;

        for e in map {
            if curr >= source_end {
                break;
            }
            if curr < e.from {
                ranges.push(Range(source.0, e.from - source.0));
                curr = e.from;
            }
            if curr >= e.from && curr < e.to {
                let end = i64::min(e.to, source_end);
                ranges.push(Range(curr + e.diff, end - curr));
                curr = end;
            }
        }
        if curr < source_end {
            ranges.push(Range(curr, source_end - curr));
        }
    }
    ranges
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
        .map(|seed| maps.iter().fold(seed, |acc, m| part_one_convert(acc, &m)))
        .min()
        .unwrap();

    println!("Part one: {}", smallest_location);
}

fn part_one_convert(source: i64, map: &Vec<Pair>) -> i64 {
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
