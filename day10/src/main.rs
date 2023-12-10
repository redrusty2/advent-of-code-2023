use std::fs;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Pos(usize, usize);
struct Offset(i32, i32);

const UP: Offset = Offset(0, -1);
const DOWN: Offset = Offset(0, 1);
const LEFT: Offset = Offset(-1, 0);
const RIGHT: Offset = Offset(1, 0);

fn main() {
    // let input = fs::read_to_string("input_example1.txt").unwrap();
    let input = fs::read_to_string("input1.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = get_start(&grid);

    println!("grid {:?}", grid);
    println!("start {:?}", start);

    let mut steps: Vec<Vec<_>> = Vec::new();
    let row_len = grid.first().unwrap().len();
    for _ in 0..grid.len() {
        steps.push(vec![-1; row_len]);
    }

    let mut prev_pos: Option<Pos> = None;
    let mut curr_pos = start;
    let mut count = 0;
    loop {
        let curr_pipe = grid[curr_pos.1][curr_pos.0];

        let curr_pipe_count = steps[curr_pos.1][curr_pos.0];
        if curr_pipe_count >= 0 {
            break;
        }

        steps[curr_pos.1][curr_pos.0] = count;
        println!("pipe {}, count {}", curr_pipe, count);

        let next_dirs = match curr_pipe {
            'S' => {
                let start_pipe = parse_start_pipe(&curr_pos, &grid);
                get_next_dirs(&curr_pos, &start_pipe)
            }
            _ => get_next_dirs(&curr_pos, &curr_pipe),
        };

        println!("dirs {:?}", next_dirs);

        for dir in next_dirs {
            if let Some(p) = &prev_pos {
                println!("next can {:?} prev {:?}", dir, prev_pos);
                if p == &dir {
                    continue;
                }
            }

            println!("chosen dir {:?}", dir);
            prev_pos = Some(curr_pos);
            curr_pos = dir;
            break;
        }

        count += 1;
    }

    // for c in steps {
    //     for i in c {
    //         print!("{:6}", i);
    //     }
    //     println!();
    // }
    //
    println!("Part one: {}", ( count + 1 ) / 2)
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

fn get_next_dirs(current_pos: &Pos, curr_pipe: &char) -> Vec<Pos> {
    let dirs = match curr_pipe {
        '|' => vec![
            add_offset(&current_pos, &UP),
            add_offset(&current_pos, &DOWN),
        ],
        '-' => vec![
            add_offset(&current_pos, &LEFT),
            add_offset(&current_pos, &RIGHT),
        ],
        'L' => vec![
            add_offset(&current_pos, &UP),
            add_offset(&current_pos, &RIGHT),
        ],
        'J' => vec![
            add_offset(&current_pos, &UP),
            add_offset(&current_pos, &LEFT),
        ],
        '7' => vec![
            add_offset(&current_pos, &DOWN),
            add_offset(&current_pos, &LEFT),
        ],
        'F' => vec![
            add_offset(&current_pos, &DOWN),
            add_offset(&current_pos, &RIGHT),
        ],
        _ => {
            panic!()
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
        let Some(new_pos) = new_pos else { panic!()};

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

    println!("mask {:?}", input);
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
                start = Some(Pos(i, j));
                break;
            }
        }
        if start.is_some() {
            break;
        }
    }
    start.unwrap().into()
}
