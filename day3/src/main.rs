use std::fs;

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    part_two(input.clone());
}

fn part_two(input: String) {
    let mut input_iter = input.lines();
    let len = input_iter.clone().count();

    let mut og_prev = "".chars().peekable();
    let mut og_curr = input_iter.next().unwrap().chars().peekable();
    let mut og_next = input_iter.next().unwrap().chars().peekable();

    let mut parts: Vec<u32> = Vec::new();

    let mut prev_symbol = false;
    for r in 0..len {
        let mut curr = og_curr.clone().enumerate();

        let mut word_has_symbol = false;
        let mut symbol = false;

        while let Some((ci, c)) = curr.next() {
            symbol = false;

            if c == '*' {
                println!("gear ci {}", ci);
                let mut numbers = Vec::new();

                let mut number = "".to_string();
                let mut is_for_gear = false;

                // search above
                let mut prev = og_prev.clone().enumerate();
                while let Some((pi, p)) = prev.next() {
                    if p.is_digit(10) {
                        number.push(p);
                        if pi >= ci - 1 && pi <= ci + 1 {
                            is_for_gear = true;
                        }
                    }
                    // end of number
                    else if number.len() > 0 {
                        if is_for_gear {
                            numbers.push(number.clone());
                        }
                        number.clear();
                        is_for_gear = false;
                    }
                }
                if number.len() > 0 {
                    if is_for_gear {
                        numbers.push(number.clone());
                    }
                    number.clear();
                    is_for_gear = false;
                }

                // search middle
                number.clear();
                is_for_gear = false;
                let mut another_curr = og_curr.clone().enumerate();
                while let Some((mi, m)) = another_curr.next() {
                    if m.is_digit(10) {
                        number.push(m);
                        if mi >= ci - 1 && mi <= ci + 1 {
                            is_for_gear = true;
                        }
                    }
                    // end of number
                    else if number.len() > 0 {
                        if is_for_gear {
                            numbers.push(number.clone());
                        }
                        number.clear();
                        is_for_gear = false;
                    }
                }
                if number.len() > 0 {
                    if is_for_gear {
                        numbers.push(number.clone());
                    }
                    number.clear();
                    is_for_gear = false;
                }

                // search below
                number.clear();
                is_for_gear = false;
                let mut next = og_next.clone().enumerate();
                while let Some((bi, b)) = next.next() {
                    if b.is_digit(10) {
                        number.push(b);
                        if bi >= ci - 1 && bi <= ci + 1 {
                            is_for_gear = true;
                        }
                    }
                    // end of number
                    else if number.len() > 0 {
                        if is_for_gear {
                            numbers.push(number.clone());
                        }
                        number.clear();
                        is_for_gear = false;
                    }
                }
                if number.len() > 0 {
                    if is_for_gear {
                        numbers.push(number.clone());
                    }
                    number.clear();
                    is_for_gear = false;
                }

                if numbers.len() == 2 {
                    println!("{:?}", numbers);
                    let mut nit = numbers.iter();
                    let product = nit.next().unwrap().parse::<u32>().unwrap()
                        * nit.next().unwrap().parse::<u32>().unwrap();
                    parts.push(product);
                }
            }
        }

        println!("line {r}");
        og_prev = og_curr;
        og_curr = og_next;
        og_next = input_iter.next().unwrap_or("").chars().peekable();
    }

    println!("parts {:?}", parts);
    println!("part one: {}", parts.iter().sum::<u32>())
}

fn part_one(input: String) {
    let mut input_iter = input.lines();
    let len = input_iter.clone().count();

    let mut og_prev = "".chars().peekable();
    let mut og_curr = input_iter.next().unwrap().chars().peekable();
    let mut og_next = input_iter.next().unwrap().chars().peekable();

    let mut parts = Vec::new();

    let mut prev_symbol = false;
    for r in 0..len {
        let mut prev = og_prev.clone();
        let mut curr = og_curr.clone();
        let mut next = og_next.clone();

        let mut number = "".to_string();
        let mut word_has_symbol = false;
        let mut symbol = false;

        while let Some(c) = curr.next() {
            symbol = false;

            // is there symbol above?
            if let Some(p) = prev.next() {
                if !p.is_digit(10) && p != '.' {
                    symbol = true;
                }
            }

            //; is there a symbol below?
            if let Some(n) = next.next() {
                if !n.is_digit(10) && n != '.' {
                    symbol = true;
                }
            }

            if c.is_digit(10) {
                number.push(c);

                if prev_symbol || symbol {
                    word_has_symbol = true;
                }
            }
            // end of number
            else {
                if c != '.' {
                    symbol = true;
                }

                if number.len() > 0 {
                    if word_has_symbol || prev_symbol || symbol {
                        println!("number {}", number);
                        parts.push(number.clone());
                    }

                    number.clear();
                    word_has_symbol = false;
                }
            }

            println!("c {}, prev {}, curr {}", c, prev_symbol, symbol);

            prev_symbol = symbol;
        }
        if number.len() > 0 {
            if word_has_symbol || prev_symbol || symbol {
                println!("number {}", number);
                parts.push(number.clone());
            }
        }

        println!("line {r}");
        og_prev = og_curr;
        og_curr = og_next;
        og_next = input_iter.next().unwrap_or("").chars().peekable();
    }

    println!("parts {:?}", parts);
    println!(
        "part one: {}",
        parts
            .iter()
            .map(|n| n.parse::<u32>().unwrap())
            .reduce(|acc, n| acc + n)
            .unwrap()
    )
}
