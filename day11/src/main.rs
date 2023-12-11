use std::fs;

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();

    part_one(input.clone());

    part_two(input);
}

fn part_two(input: String) {
    let time_factor = 1000000;

    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    print_grid(&grid);

    let mut empty_rows: Vec<_> = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(y, _)| y)
        .collect();
    empty_rows.sort();

    let mut empty_cols = Vec::new();
    let mut col_idx = 0;
    while col_idx < grid.first().unwrap().len() {
        let mut empty = true;
        for row_idx in 0..grid.len() {
            let cell = grid[row_idx][col_idx];
            if cell == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            empty_cols.push(col_idx);
        }
        col_idx += 1;
    }
    empty_cols.sort();

    let gxs: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if cell == &'#' {
                    return Some((x.clone(), y.clone()));
                }
                None
            })
        })
        .collect();

    println!("{:?}", gxs);

    let mut distances: Vec<(usize, usize)> = Vec::new();
    for i in 0..gxs.len() {
        for j in i + 1..gxs.len() {
            let a = gxs[i];
            let b = gxs[j];
            let from_x = usize::min(a.0, b.0);
            let from_y = usize::min(a.1, b.1);
            let to_x = usize::max(a.0, b.0);
            let to_y = usize::max(a.1, b.1);

            let ec = empty_cols
                .iter()
                .filter(|&&c| c >= from_x && c <= to_x)
                .count();
            let er = empty_rows
                .iter()
                .filter(|&&r| r >= from_y && r <= to_y)
                .count();
            distances.push((
                a.0.abs_diff(b.0) + ec * time_factor - ec,
                a.1.abs_diff(b.1) + er * time_factor - er,
            ));
        }
    }

    let sum: usize = distances.iter().map(|(x, y)| x + y).sum();

    println!("Part two: {}", sum);
}

fn part_one(input: String) {
    let mut expanded: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    print_grid(&expanded);
    expand(&mut expanded);
    print_grid(&expanded);

    let gxs: Vec<_> = expanded
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, cell)| {
                if cell == &'#' {
                    return Some((x.clone(), y.clone()));
                }
                None
            })
        })
        .collect();

    println!("{:?}", gxs);

    let mut distances = Vec::new();
    for i in 0..gxs.len() {
        for j in i + 1..gxs.len() {
            let a = gxs[i];
            let b = gxs[j];
            let dist = (a.0.abs_diff(b.0), a.1.abs_diff(b.1));
            distances.push(dist);
        }
    }

    println!("distances {:?}", distances);

    print_grid(&expanded);

    let sum: usize = distances.iter().map(|(x, y)| x + y).sum();

    println!("Part one: {}", sum);
}

fn expand(grid: &mut Vec<Vec<char>>) {
    let mut row_idx = 0;
    while row_idx < grid.len() {
        let row = &grid[row_idx];
        if !row.contains(&'#') {
            grid.insert(row_idx, row.clone());
            row_idx += 1
        }
        row_idx += 1;
    }

    let mut col_idx = 0;
    while col_idx < grid.first().unwrap().len() {
        let mut empty = true;
        for row_idx in 0..grid.len() {
            let cell = grid[row_idx][col_idx];
            if cell == '#' {
                empty = false;
                break;
            }
        }
        if empty {
            for j in 0..grid.len() {
                grid[j].insert(col_idx, '.');
            }
            col_idx += 1
        }
        col_idx += 1;
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
