use std::fs;

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();

    let init: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut rolled = init.clone();

    print_grid(&init);

    for x in 0..rolled.first().unwrap().len() {
        for y in 0..rolled.len() {
            let curr = rolled[y][x];
            // println!("================ x {} y {} curr {}", x, y, rolled[y][x]);

            if curr == 'O' {
                roll_rock(x, y, &mut rolled);
            }
        }
    }

    print_grid(&rolled);

    let sum: usize = rolled
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let cnt = row.iter().filter(|&r| r == &'O').count();
            println!("cnt {} i {}", cnt, i);
            cnt * (rolled.len() - i)
        })
        .sum();

    println!("Part one: {}", sum);
}

fn roll_rock(x: usize, y: usize, rolled: &mut Vec<Vec<char>>) {
    let mut mi = 0;
    for i in (0..y).rev() {
        let c = rolled[i][x];
        // println!("x {} yi {} c {}", x, i, c);
        if c != '.' {
            mi = i + 1;
            break;
        }
    }
    if mi < y {
        // println!("move y {} to mi {}", y, mi);
        rolled[mi][x] = 'O';
        rolled[y][x] = '.';
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!()
}
