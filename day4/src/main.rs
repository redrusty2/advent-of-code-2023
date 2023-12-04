use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    part_one(input.clone());
    part_two(input);
}

fn part_two(input: String) {
    let mut copies: HashMap<usize, usize> = HashMap::new();
    let mut count: usize = 0;
    input.lines().for_each(|line| {
        let mut split = line.split(":").flat_map(|s| s.split("|"));
        let game = split
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let winners: HashSet<&str> = HashSet::from_iter(split.next().unwrap().split_whitespace());
        let ours = HashSet::from_iter(split.next().unwrap().split_whitespace());

        let matches = winners.intersection(&ours).count();

        // run for copies
        let c = copies.get(&game).unwrap_or(&1);
        for _ in 0..*c {
            count += 1;

            // create copies
            for i in game + 1..game + matches + 1 {
                let x = copies.get_mut(&i);
                if let Some(v) = x {
                    *v += 1;
                } else {
                    copies.insert(i, 2);
                }
            }
        }
    });

    println!("Part two: {}", count);
}

fn part_one(input: String) {
    let result = input
        .lines()
        .map(|line| {
            let mut split = line.split(":").nth(1).unwrap().split("|").map(|s| s.trim());
            let winners: HashSet<&str> =
                HashSet::from_iter(split.next().unwrap().split_whitespace());
            let ours = HashSet::from_iter(split.next().unwrap().split_whitespace());

            let count = winners.intersection(&ours).count();

            if count == 0 {
                return 0;
            }
            usize::pow(2, (count - 1).try_into().unwrap())
        })
        .reduce(|acc, n| acc + n)
        .unwrap();

    println!("Part one: {}", result);
}
