use std::fs;

fn main() {
    let input = fs::read_to_string("input_example1.txt").unwrap();

    part_one(input.clone());
    part_two(input);
}

fn part_two(input: String) {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: u64 = lines
        .next()
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .unwrap();

    println!("time {time}, dist {distance}");


    let mut wins = 0;
    for speed in 1..time - 1 {
        let time_left = time - speed;
        let result = speed * time_left;
        if result > distance {
            wins += 1;
        }
    }

    println!("Part two: {}", wins);
}

fn part_one(input: String) {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u32>().unwrap());

    let result: u64 = times
        .zip(distances)
        .map(|(t, d)| {
            let mut wins = 0;
            for speed in 1..t - 1 {
                let time_left = t - speed;
                let result = speed * time_left;
                if result > d {
                    wins += 1;
                }
            }
            wins
        })
        .product();

    println!("Part one: {}", result);
}
