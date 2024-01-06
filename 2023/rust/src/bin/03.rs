use std::io::{self, BufRead};

fn is_symbol(c: char) -> bool {
    !(c.is_numeric() || c == '.')
}

fn get_value(source: &Vec<char>, width: usize, x: usize, y: usize) -> Option<&char> {
    if x < width + 1 {
        return source.get(y * (width + 1) + x);
    }

    None
}

fn get_close_numbers(source: &Vec<char>, width: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut r = Vec::<(usize, usize)>::new();

    let mut neigh = Vec::<(usize, usize)>::new();

    if x > 0 {
        neigh.push((x - 1, y));
        neigh.push((x - 1, y + 1));
        if y > 0 {
            neigh.push((x - 1, y - 1))
        }
    }

    if y > 0 {
        neigh.push((x, y - 1));
        neigh.push((x + 1, y - 1));
    }

    neigh.push((x, y + 1));
    neigh.push((x + 1, y));
    neigh.push((x + 1, y + 1));

    for (x, y) in neigh {
        if let Some(c) = get_value(source, width, x, y) {
            if c.is_numeric() {
                r.push((x, y))
            }
        }
    }

    r
}

fn clean_number(source: &mut Vec<char>, start: usize, number: u32) {
    let s_number = number.to_string();

    for x in 0..s_number.len() {
        source[start + x] = '.';
    }
}

fn fetch_number(source: &mut Vec<char>, width: usize, x: usize, y: usize) -> u32 {
    let mut start = y * (width + 1) + x;

    while let Some(x) = source.get(start) {
        if start > 0 && x.is_numeric() {
            start -= 1;
        } else {
            break;
        }
    }

    if start > 0 {
        start += 1;
    }

    let t_source = &source[start..];
    let number = t_source
        .iter()
        .map_while(|c| c.to_digit(10))
        .fold(0, |acc, d| acc * 10 + d);

    clean_number(source, start, number);

    number
}

fn main() {
    let mut schematics = Vec::<char>::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut symbols = Vec::<(usize, usize)>::new();
    // let mut starting_number = Vec::<(usize, usize)>::new();
    let mut total: u32 = 0;
    let mut gears_ratio: u32 = 0;

    for line in io::stdin().lock().lines() {
        let mut x: usize = 0;
        for c in line.unwrap().trim().chars() {
            schematics.push(c);
            if is_symbol(c) {
                symbols.push((x, height))
            }
            x += 1;
        }
        height += 1;
        width = x
    }

    // height -= 1;
    width -= 1;

    // println!("{:?}", schematics);
    // println!("symbols {:?}", symbols);
    // println!("sizes {:?}", (width, height));

    for (x, y) in symbols {
        let mut c_numbers = Vec::<u32>::new();
        for (x, y) in get_close_numbers(&schematics, width, x, y) {
            let n = fetch_number(&mut schematics, width, x, y);
            c_numbers.push(n);
            total += n;
        }

        c_numbers.retain(|&x| x != 0);
        if let Some(s) = get_value(&schematics, width, x, y) {
            if *s == '*' && c_numbers.len() == 2 {
                gears_ratio += c_numbers.iter().fold(1, |acc, n| acc * n);
            }
        }
    }

    // println!("numbers {:?}", starting_number);

    // println!("{:?}", fetch_number(&mut schematics, width, 2, 2));
    // println!("{:?}", schematics);

    println!("{}", total);
    println!("{}", gears_ratio);
}
