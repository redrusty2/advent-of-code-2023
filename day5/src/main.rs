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
        .map(|n| n.parse::<i64>().unwrap()).collect();

    lines.nth(1);
    let seed_to_soil = create_map(&mut lines);

    lines.next();
    let soil_to_fertilizer = create_map(&mut lines);

    lines.next();
    let fertilizer_to_water = create_map(&mut lines);

    lines.next();
    let water_to_light = create_map(&mut lines);

    lines.next();
    let light_to_temp = create_map(&mut lines);

    lines.next();
    let temp_to_humid = create_map(&mut lines);

    lines.next();
    let humid_to_location = create_map(&mut lines);

    let locations: Vec<i64> = seeds
        .chunks(2)
        .inspect(|w| println!("{} {}", w[0], w[1]))
        .flat_map(|w| {
            (w[0]..w[0] + w[1])
                .into_iter()
                .map(|seed| convert(seed, &seed_to_soil))
                .map(|soil| convert(soil, &soil_to_fertilizer))
                .map(|fert| convert(fert, &fertilizer_to_water))
                .map(|water| convert(water, &water_to_light))
                .map(|light| convert(light, &light_to_temp))
                .map(|temp| convert(temp, &temp_to_humid))
                .map(|humid| convert(humid, &humid_to_location))
        })
        .collect();

    println!("Part two: {}", locations.iter().min().unwrap());
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

    lines.nth(1);
    let seed_to_soil = create_map(&mut lines);

    lines.nth(1);
    let soil_to_fertilizer = create_map(&mut lines);

    lines.nth(1);
    let fertilizer_to_water = create_map(&mut lines);

    lines.nth(1);
    let water_to_light = create_map(&mut lines);

    lines.nth(1);
    let light_to_temp = create_map(&mut lines);

    lines.nth(1);
    let temp_to_humid = create_map(&mut lines);

    lines.nth(1);
    let humid_to_location = create_map(&mut lines);

    let locations: Vec<i64> = seeds
        .map(|seed| convert(seed, &seed_to_soil))
        .map(|soil| convert(soil, &soil_to_fertilizer))
        .map(|fert| convert(fert, &fertilizer_to_water))
        .map(|water| convert(water, &water_to_light))
        .map(|light| convert(light, &light_to_temp))
        .map(|temp| convert(temp, &temp_to_humid))
        .map(|humid| convert(humid, &humid_to_location))
        .collect();

    println!("Part one: {}", locations.iter().min().unwrap());
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
