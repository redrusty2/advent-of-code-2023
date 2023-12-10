use std::{collections::VecDeque, fs};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct Pos(usize, usize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Offset(i32, i32);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_up(&self) -> bool {
        self == &Direction::Up
    }
    fn is_down(&self) -> bool {
        self == &Direction::Down
    }
    fn is_left(&self) -> bool {
        self == &Direction::Left
    }
    fn is_right(&self) -> bool {
        self == &Direction::Right
    }
}

const ABOVE: Offset = Offset(0, -1);
const ABOVE_BEFORE: Offset = Offset(-1, -1);
const ABOVE_AFTER: Offset = Offset(1, -1);
const BELOW: Offset = Offset(0, 1);
const BELOW_BEFORE: Offset = Offset(-1, 1);
const BELOW_AFTER: Offset = Offset(1, 1);
const BEFORE: Offset = Offset(-1, 0);
const AFTER: Offset = Offset(1, 0);

fn main() {
    // let input = fs::read_to_string("my_input.txt").unwrap();
    // let input = fs::read_to_string("input_example2.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = get_start(&grid);

    println!("grid {:?}", grid);
    println!("start {:?}", start);

    let mut steps: Vec<Vec<_>> = Vec::new();
    let mut area: Vec<Vec<_>> = Vec::new();
    let row_len = grid.first().unwrap().len();
    for _ in 0..grid.len() {
        steps.push(vec![-1; row_len]);
        area.push(vec!['.'; row_len]);
    }
    let mut path: Vec<Pos> = Vec::new();

    let mut prev_pos: Option<Pos> = None;
    let mut curr_pos = start;
    let mut count = 0;
    loop {
        let curr_pipe = match grid[curr_pos.1][curr_pos.0] {
            'S' => parse_start_pipe(&curr_pos, &grid),
            cp => cp,
        };

        let curr_pipe_count = steps[curr_pos.1][curr_pos.0];
        if curr_pipe_count >= 0 {
            break;
        }
        path.push(curr_pos.clone());

        steps[curr_pos.1][curr_pos.0] = count;

        // get possible next position
        let next_positons = get_next_dirs(&curr_pos, &curr_pipe);

        // choose next position
        for possible_pos in next_positons {
            if let Some(p) = &prev_pos {
                if p == &possible_pos {
                    continue;
                }
            }

            prev_pos = Some(curr_pos);
            curr_pos = possible_pos;
            break;
        }
        count += 1;
    }

    // paint the sides
    paint_sides(&path, &grid, &mut area);

    // combine area with grid
    let mut area_grid = grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            let a = area[y][x];
            if a != '.' {
                area_grid[y][x] = a;
            } 
            // overwrite non path pipes
            else if !path.contains(&Pos(x, y)) {
                area_grid[y][x] = '.';
            }
        }
    }

    // fill area mask gaps
    // search for . and then recursively expand until area marker found, then replace all . with
    // the marker.
    for y in 0..area_grid.len() {
        for x in 0..area_grid.first().unwrap().len() {
            let m = area_grid[y][x];
            if m == '.' {
                paint(&Pos(x, y), &mut area_grid);
            }
        }
    }

    for row in area_grid.clone() {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    println!();

    println!("Part one: {}", (count + 1) / 2);

    // TODO determine inside and out from number of turns
    let mut x_count = 0;
    for row in area_grid {
        for cell in row {
            if cell == 'X' {
                x_count += 1;
            }
        }
    }
    println!("Part two: {}", x_count);
}

fn paint_sides(path: &Vec<Pos>, grid: &Vec<Vec<char>>, area: &mut Vec<Vec<char>>) {
    let mut curr_pos = path.first().unwrap();
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    for i in 0..path.len() - 1 {
        let curr_pipe = match grid[curr_pos.1][curr_pos.0] {
            'S' => parse_start_pipe(&curr_pos, &grid),
            cp => cp,
        };

        let prev_pos = curr_pos;
        curr_pos = &path[i + 1];

        // write to area mask
        // NOTICE this is after prev and curr and been advanced
        // get our direction
        let d = Offset(
            curr_pos.0 as i32 - prev_pos.0 as i32,
            curr_pos.1 as i32 - prev_pos.1 as i32,
        );
        let direction = match d {
            ABOVE => {
                up += 1;
                Direction::Up
            }
            BELOW => {
                down += 1;
                Direction::Down
            }
            BEFORE => {
                left += 1;
                Direction::Left
            }
            AFTER => {
                right += 1;
                Direction::Right
            }
            _ => {
                panic!()
            }
        };

        write_area(&curr_pipe, &prev_pos, &path, area, &direction);
    }

    println!("up {}, down {}, left {}, right {}", up, down, left, right);
}

fn paint(start: &Pos, area: &mut Vec<Vec<char>>) {
    let offsets = vec![ABOVE, BELOW, BEFORE, AFTER];
    // let mut edge: HashSet<Pos> = HashSet::new();
    // let mut empty = Vec::new();
    let mut empty: Vec<Pos> = Vec::new();
    let mut edge: VecDeque<Pos> = VecDeque::new();
    let mut marker_type: char = 'E';
    let row_len = area.first().unwrap().len();

    edge.push_back(*start);

    while !edge.is_empty() {
        let pos = edge.pop_front().unwrap();
        let m = area[pos.1][pos.0];

        match m {
            '.' => {
                empty.push(pos);
            }
            'O' | 'X' => {
                marker_type = m;
                continue;
            }
            _ => {
                continue;
            }
        }

        // add next positons to edge
        for off in &offsets {
            let next_pos_opt = add_offset(&pos, &off);

            if let Some(next) = next_pos_opt {
                if next.0 < row_len
                    && next.1 < area.len()
                    && !edge.contains(&next)
                    && !empty.contains(&next)
                {
                    edge.push_back(next);
                }
            }
        }
    }

    for pos in empty {
        area[pos.1][pos.0] = marker_type;
    }
}

fn write_area(
    pipe: &char,
    pos: &Pos,
    path: &Vec<Pos>,
    area: &mut Vec<Vec<char>>,
    from: &Direction,
) {
    match pipe {
        '|' => {
            write_area_pos(&pos, &BEFORE, &path, area, from.is_up());
            write_area_pos(&pos, &AFTER, &path, area, from.is_down());
        }
        '-' => {
            write_area_pos(&pos, &ABOVE, &path, area, from.is_right());
            write_area_pos(&pos, &BELOW, &path, area, from.is_left());
        }
        'L' => {
            write_area_pos(&pos, &BEFORE, &path, area, !from.is_right());
            write_area_pos(&pos, &BELOW, &path, area, !from.is_right());
            write_area_pos(&pos, &BELOW_BEFORE, &path, area, !from.is_right());
        }
        'J' => {
            write_area_pos(&pos, &AFTER, &path, area, from.is_left());
            write_area_pos(&pos, &BELOW, &path, area, from.is_left());
            write_area_pos(&pos, &BELOW_AFTER, &path, area, from.is_left());
        }
        '7' => {
            write_area_pos(&pos, &AFTER, &path, area, !from.is_left());
            write_area_pos(&pos, &ABOVE, &path, area, !from.is_left());
            write_area_pos(&pos, &ABOVE_AFTER, &path, area, !from.is_left());
        }
        'F' => {
            write_area_pos(&pos, &BEFORE, &path, area, from.is_right());
            write_area_pos(&pos, &ABOVE, &path, area, from.is_right());
            write_area_pos(&pos, &ABOVE_BEFORE, &path, area, from.is_right());
        }
        _ => {}
    }
}

fn write_area_pos(
    pos: &Pos,
    offset: &Offset,
    path: &Vec<Pos>,
    area: &mut Vec<Vec<char>>,
    alt: bool,
) {
    let adj = add_offset(&pos, &offset);
    if let Some(p) = adj {
        if !path.contains(&p) {
            area[p.1][p.0] = if alt { 'X' } else { 'O' };
        }
    }
}

fn add_offset(pos: &Pos, offset: &Offset) -> Option<Pos> {
    let rx: Result<usize, _> = (pos.0 as i32 + offset.0).try_into();
    let ry: Result<usize, _> = (pos.1 as i32 + offset.1).try_into();

    if rx.is_err() || ry.is_err() {
        return None;
    }

    let Ok(x) = rx else { panic!() };
    let Ok(y) = ry else { panic!() };

    Some(Pos(x, y))
}

// TODO take in direction
fn get_next_dirs(current_pos: &Pos, curr_pipe: &char) -> Vec<Pos> {
    let dirs = match curr_pipe {
        '|' => vec![
            add_offset(&current_pos, &ABOVE),
            add_offset(&current_pos, &BELOW),
        ],
        '-' => vec![
            add_offset(&current_pos, &BEFORE),
            add_offset(&current_pos, &AFTER),
        ],
        'L' => vec![
            add_offset(&current_pos, &ABOVE),
            add_offset(&current_pos, &AFTER),
        ],
        'J' => vec![
            add_offset(&current_pos, &ABOVE),
            add_offset(&current_pos, &BEFORE),
        ],
        '7' => vec![
            add_offset(&current_pos, &BELOW),
            add_offset(&current_pos, &BEFORE),
        ],
        'F' => vec![
            add_offset(&current_pos, &BELOW),
            add_offset(&current_pos, &AFTER),
        ],
        '.' => {
            vec![]
        }
        _ => {
            panic!("curr pipe {}", curr_pipe);
        }
    };

    dirs.into_iter().filter_map(|d| d).collect()
}

fn parse_start_pipe(pos: &Pos, grid: &Vec<Vec<char>>) -> char {
    let offsets: Vec<Offset> = vec![Offset(0, -1), Offset(-1, 0), Offset(1, 0), Offset(0, 1)];

    let mut input = vec![false, false, false, false];

    for (i, off) in offsets.iter().enumerate() {
        let new_pos = add_offset(&pos, &off);

        if new_pos.is_none() {
            continue;
        }
        let Some(new_pos) = new_pos else { panic!() };

        let curr = grid[new_pos.1][new_pos.0];

        if i == 0 {
            match curr {
                '|' | '7' | 'F' => input[i] = true,
                _ => {}
            }
        } else if i == 1 {
            match curr {
                '-' | 'L' | 'F' => input[i] = true,
                _ => {}
            }
        } else if i == 2 {
            match curr {
                '-' | 'J' | '7' => input[i] = true,
                _ => {}
            }
        } else if i == 3 {
            match curr {
                '|' | 'L' | 'J' => input[i] = true,
                _ => {}
            }
        }
    }

    match input[..] {
        [true, false, false, true] => '|',
        [false, true, true, false] => '-',
        [true, false, true, false] => 'L',
        [true, true, false, false] => 'J',
        [false, true, false, true] => '7',
        [false, false, true, true] => 'F',
        _ => {
            panic!()
        }
    }
}

fn get_start(grid: &Vec<Vec<char>>) -> Pos {
    let mut start: Option<Pos> = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'S' {
                start = Some(Pos(j, i));
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    start.unwrap().into()
}
