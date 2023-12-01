use std::{
    cmp::min,
    fs::{self},
};

fn main() {
    let input = fs::read_to_string("input1.txt").expect("Something went wrong reading the file");

    part_one(input.clone());
    part_two(input);
}

fn part_two(input: String) {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let win_size = 6;
    let calibrate: u32 = input
        .lines()
        .map(|line| {
            let mut digits = String::new();

            let mut citer = line.char_indices().peekable();
            while let Some((i, c)) = citer.next() {
                if c.is_digit(10) {
                    digits.push(c);
                }
                let end = min(line.len(), i + win_size + 1);
                let slc = line.get(i..end).unwrap();

                for (i, n) in numbers.iter().enumerate() {
                    if slc.starts_with(*n) {
                        digits.push_str(&(i + 1).to_string());
                        for _ in 0..n.len(){
                            citer.next();
                        }
                        continue;
                    }
                }
            }

            if digits.len() == 1 {
                digits.push_str(&digits.clone());
            }
            let mut a = String::new();
            let mut it = digits.chars();
            a.push(it.next().unwrap());
            a.push(it.last().unwrap());

            let result = a.parse::<u32>().unwrap();

            // println!("result {}, digits {}", result, digits);
            result
        })
        .reduce(|acc, item| acc + item)
        .unwrap();

    println!("Part two: {}", calibrate);
}

fn part_one(input: String) {
    let calibrate: u32 = input
        .lines()
        .map(|line| {
            let mut digits = String::new();

            for c in line.chars() {
                if c.is_digit(10) {
                    digits.push(c);
                }
            }

            if digits.len() == 1 {
                digits.push_str(&digits.clone());
            }
            let mut a = String::new();
            let mut it = digits.chars();
            a.push(it.next().unwrap());
            a.push(it.last().unwrap());

            a.parse::<u32>().unwrap()
        })
        .reduce(|acc, item| acc + item)
        .unwrap();

    println!("Part one: {}", calibrate);
}
