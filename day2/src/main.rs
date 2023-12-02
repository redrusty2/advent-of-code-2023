use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    part_one(input.clone());
    part_two(input);
}

fn part_two(input: String) {
    let mut acc = 0;

    input.lines().for_each(|line| {
        let mut lsplit = line.split(":");
        let rounds = lsplit.nth(1).unwrap().split(";");

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for r in rounds {
            for cube in r.split(",").map(|c| c.trim()) {
                let mut x = cube.split(" ");
                let n: u32 = x.next().unwrap().parse().unwrap();
                let c = x.next().unwrap();

                match c {
                    "red" => {
                        if n > min_red {
                            min_red = n;
                        }
                    }
                    "green" => {
                        if n > min_green {
                            min_green = n;
                        }
                    }
                    "blue" => {
                        if n > min_blue {
                            min_blue = n;
                        }
                    }
                    _ => {}
                }
            }
        }
        acc += min_red * min_blue * min_green;
    });

    println!("Part two: {}", acc);
}

fn part_one(input: String) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut valid_games_acc = 0;

    input.lines().for_each(|line| {
        let mut lsplit = line.split(":");
        let game: u32 = lsplit
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let rounds = lsplit.next().unwrap().split(";");

        let mut valid = true;
        for r in rounds {
            for cube in r.split(",").map(|c| c.trim()) {
                let mut x = cube.split(" ");
                let n: u32 = x.next().unwrap().parse().unwrap();
                let c = x.next().unwrap();

                match c {
                    "red" => {
                        if n > max_red {
                            valid = false;
                            break;
                        }
                    }
                    "green" => {
                        if n > max_green {
                            valid = false;
                            break;
                        }
                    }
                    "blue" => {
                        if n > max_blue {
                            valid = false;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if !valid {
                break;
            }
        }

        if valid {
            valid_games_acc += game;
        }
    });

    println!("Part one: {}", valid_games_acc)
}
