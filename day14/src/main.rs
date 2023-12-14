use std::fs;

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    // let input = fs::read_to_string("my_input.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();

    let init: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut rolled = init.clone();

    print_grid(&init);

    let mut sums = Vec::new();
    for _ in 0..1000 {
        // north
        for x in 0..rolled.first().unwrap().len() {
            for y in 0..rolled.len() {
                let curr = rolled[y][x];

                if curr == 'O' {
                    roll_rock_north(x, y, &mut rolled);
                }
            }
        }

        // west
        for y in 0..rolled.len() {
            for x in 0..rolled.first().unwrap().len() {
                let curr = rolled[y][x];

                if curr == 'O' {
                    roll_rock_west(x, y, &mut rolled);
                }
            }
        }

        // south
        for x in 0..rolled.first().unwrap().len() {
            for y in (0..rolled.len()).rev() {
                let curr = rolled[y][x];

                if curr == 'O' {
                    roll_rock_south(x, y, &mut rolled);
                }
            }
        }

        // east
        for y in 0..rolled.len() {
            for x in (0..rolled.first().unwrap().len()).rev() {
                let curr = rolled[y][x];

                if curr == 'O' {
                    roll_rock_east(x, y, &mut rolled);
                }
            }
        }
        let sum: usize = rolled
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let cnt = row.iter().filter(|&r| r == &'O').count();
                cnt * (rolled.len() - i)
            })
            .sum();
        sums.push(sum);
    }

    let mut pattern_start = 0;
    let mut patten_len = 0;
    for i in 0..sums.len() {
        let len = search_for_pattern(i, &sums);
        if len > 0 {
            pattern_start = i;
            patten_len = len;
            break;
        }
    }
    if patten_len == 0 {
        println!("No pattern found");
        return;
    }

    let cycles = 1_000_000_000;
    let i = (cycles - pattern_start) % patten_len + pattern_start -1;
    let sum = sums[i];

    println!("Part two: {}", sum);
}

fn search_for_pattern(start: usize, list: &[usize]) -> usize {
    let min_len = 2;
    let mut second_start = start;
    while second_start < list.len() {
        let mut second_occur = second_start;

        // find next occurence
        let mut len = 1;
        for i in second_start + 1..list.len() {
            if list[i] == list[start] && len >= min_len {
                second_occur = i;
                break;
            }
            len += 1;
        }
        if second_occur == second_start {
            break;
        }
        second_start = second_occur;

        // Match pattern
        let mut a = start + 1;
        let mut b = second_occur + 1;
        let mut pattern_invalid = false;
        while a < second_occur && b < list.len() {
            if list[a] != list[b] {
                pattern_invalid = true;
                break;
            }
            a += 1;
            b += 1;
        }
        if pattern_invalid {
            continue;
        }

        // Did the loop end because of a pattern or b reaching the end of the list?
        let pattern_len = second_occur - start;
        let pattern_valid = a == second_occur && b == second_occur + pattern_len;
        if !pattern_valid {
            continue;
        }

        // Test the pattern occurs again
        let mut test_start = start;
        let mut pattern_valid = true;
        while test_start + pattern_len < list.len() {
            if !test_pattern_valid(test_start, pattern_len, list) {
                pattern_valid = false;
                break;
            }

            test_start += pattern_len;
        }
        if pattern_valid {
            return pattern_len;
        }
    }
    return 0;
}

fn test_pattern_valid(start: usize, len: usize, list: &[usize]) -> bool {
    let mut a = start + 1;
    let mut b = start + len + 1;
    while a < start + len && b < list.len() {
        if list[a] != list[b] {
            return false;
        }
        a += 1;
        b += 1;
    }
    if a == start + len && b == start + len + len {
        return true;
    }
    false
}

fn roll_rock_north(x: usize, y: usize, rolled: &mut Vec<Vec<char>>) {
    if y == 0 {
        return;
    }
    let mut i = y - 1;
    while i < y {
        let c = rolled[i][x];
        if c != '.' {
            if i < y - 1 {
                rolled[i + 1][x] = 'O';
                rolled[y][x] = '.';
            }
            return;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    if y != 0 {
        rolled[y][x] = '.';
        rolled[0][x] = 'O';
    }
}

fn roll_rock_south(x: usize, y: usize, rolled: &mut Vec<Vec<char>>) {
    let mut i = y + 1;
    while i < rolled.len() {
        let c = rolled[i][x];
        if c != '.' {
            if i > y + 1 {
                rolled[i - 1][x] = 'O';
                rolled[y][x] = '.';
            }
            return;
        }
        i += 1;
    }
    let end = rolled.len() - 1;
    if end != y {
        rolled[y][x] = '.';
        rolled[end][x] = 'O';
    }
}

fn roll_rock_east(x: usize, y: usize, rolled: &mut Vec<Vec<char>>) {
    let mut i = x + 1;
    let row_len = rolled.first().unwrap().len();
    while i < row_len {
        let c = rolled[y][i];
        if c != '.' {
            if i > x + 1 {
                rolled[y][i - 1] = 'O';
                rolled[y][x] = '.';
            }
            return;
        }
        i += 1;
    }
    if x != row_len - 1 {
        rolled[y][x] = '.';
        rolled[y][row_len - 1] = 'O';
    }
}

fn roll_rock_west(x: usize, y: usize, rolled: &mut Vec<Vec<char>>) {
    if x == 0 {
        return;
    }
    let mut i = x - 1;
    loop {
        let c = rolled[y][i];
        if c != '.' {
            if i < x - 1 {
                rolled[y][i + 1] = 'O';
                rolled[y][x] = '.';
            }
            return;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    if x != 0 {
        rolled[y][x] = '.';
        rolled[y][0] = 'O';
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
