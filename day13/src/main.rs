use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();
    // let input = fs::read_to_string("my_input.txt").unwrap();

    let sum: usize = input
        .split("\n\n")
        .map(|pattern| {
            let vert_refl = find_vert_refl(pattern);
            if vert_refl == 0 {
                return find_hor_refl(pattern);
            }
            vert_refl
        })
        .sum();

    println!("Part one {}", sum);
}

fn find_hor_refl(pattern: &str) -> usize {
    let grid: Vec<Vec<char>> = pattern.lines().map(|line| line.chars().collect()).collect();

    let mut rows: HashSet<usize> = HashSet::new();
    let mut all_sets = Vec::new();
    for i in 0..grid.first().unwrap().len() {
        let col: Vec<char> = grid.iter().map(|g| g[i]).collect();
        let mut curr_rows: HashSet<usize> = HashSet::new();

        // loop each pivot
        for p in 0..col.len() {
            let refl = check_reflection(p, &col);

            // reflection found
            if refl {
                curr_rows.insert(p);
            }
        }

        all_sets.push(curr_rows.clone());
    }

    all_sets
        .into_iter()
        .fold(HashMap::new(), |mut acc, set| {
            for s in set {
                let count = acc.entry(s).or_insert(0);
                *count += 1;
            }
            acc
        })
        .into_iter()
        .filter(|(_, v)| *v == pattern.lines().next().unwrap().len() - 1)
        .map(|(k, _)| (k + 1) * 100)
        .sum()
}

fn find_vert_refl(pattern: &str) -> usize {
    let mut all_sets = Vec::new();
    for line in pattern.lines(){
        let line_slice: Vec<char> = line.chars().collect();
        let mut curr_cols: HashSet<usize> = HashSet::new();

        // loop each pivot
        for p in 0..line.len() {
            let refl = check_reflection(p, &line_slice);

            // reflection found
            if refl {
                curr_cols.insert(p);
            }
        }

        all_sets.push(curr_cols.clone());
    }

    all_sets
        .into_iter()
        .fold(HashMap::new(), |mut acc, set| {
            for s in set {
                let count = acc.entry(s).or_insert(0);
                *count += 1;
            }
            acc
        })
        .into_iter()
        .filter(|(_, v)| *v == pattern.lines().count() - 1)
        .map(|(k, _)| k + 1)
        .sum()
}

fn check_reflection(p: usize, line: &[char]) -> bool {
    if p >= line.len() - 1 {
        return false;
    }
    let mut i: i32 = p.try_into().unwrap();
    let mut j = p + 1;

    while i >= 0 && j < line.len() {
        if line[i as usize] != line[j] {
            return false;
        }

        i -= 1;
        j += 1;
    }

    true
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
