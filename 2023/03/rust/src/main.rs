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

fn main() {
    let mut schematics = Vec::<char>::new();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut symbols = Vec::<(usize, usize)>::new();
    let mut starting_number = Vec::<(usize, usize)>::new();

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

    height -= 1;
    width -= 1;

    println!("{:?}", schematics);
    println!("symbols {:?}", symbols);
    println!("sizes {:?}", (width, height));

    for (x, y) in symbols {
        starting_number.extend(get_close_numbers(&schematics, width, x, y));
    }

    println!("numbers {:?}", starting_number);
    println!("test {:?}", get_close_numbers(&schematics, width, 1, 0));
}
