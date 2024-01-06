use std::{
    cmp::{max, min},
    io,
};

fn _display_map(map: &Vec<char>, width: usize, height: usize) {
    let mut r: String = "".to_string();

    for (i, &ch) in map.iter().enumerate() {
        if i % width == 0 {
            r.push('\n');
        }
        r.push(ch);
    }

    println!("{}\nw{} h{}", r.trim(), width, height)
}

fn get_expand(map: &Vec<char>, width: usize, height: usize) -> (Vec<usize>, Vec<usize>) {
    let mut exp_y: Vec<usize> = Vec::new();
    let mut exp_x: Vec<usize> = Vec::new();

    let empty_row = ".".repeat(width);
    for y in 0..height {
        let start = y * width;
        let slice: String = map
            .get(start..start + width)
            .expect("Should always find complete line")
            .iter()
            .collect();
        if slice == empty_row {
            exp_y.push(y);
        }
    }

    let empty_col = ".".repeat(height);
    for x in 0..width {
        let slice: String = map
            .iter()
            .enumerate()
            .filter(|(i, _)| i % width == x)
            .map(|(_, ch)| ch)
            .collect();
        if slice == empty_col {
            exp_x.push(x)
        }
    }

    (exp_x, exp_y)
}

fn get_distance(
    a: (usize, usize),
    b: (usize, usize),
    exp_x: &Vec<usize>,
    exp_y: &Vec<usize>,
    exp_factor: usize,
) -> usize {
    let x_min = min(a.0, b.0);
    let x_max = max(a.0, b.0);

    let y_min = min(a.1, b.1);
    let y_max = max(a.1, b.1);

    let mut d_x = x_max - x_min;
    let mut d_y = y_max - y_min;

    for x in exp_x {
        if (x_min..x_max).contains(x) {
            d_x += exp_factor - 1;
        }
    }

    for y in exp_y {
        if (y_min..y_max).contains(y) {
            d_y += exp_factor - 1;
        }
    }

    d_x + d_y
}

fn main() {
    let mut width: usize = usize::MAX;
    let mut grid: Vec<char> = Vec::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (y, line) in io::stdin().lines().enumerate() {
        let c_line = line.expect("Should have valid lines");
        width = min(width, c_line.len());
        for (x, ch) in c_line.trim().char_indices() {
            grid.push(ch);
            if ch == '#' {
                galaxies.push((x, y))
            }
        }
    }

    let height = grid.len() / width;
    // _display_map(&grid, width, height);
    let (exp_x, exp_y) = get_expand(&grid, width, height);

    let mut t_distance2: usize = 0;
    // let mut t_distance10: usize = 0;
    // let mut t_distance100: usize = 0;
    let mut t_distance1000000: usize = 0;

    for a in 0..galaxies.len() - 1 {
        for b in a + 1..galaxies.len() {
            let g_a = galaxies[a];
            let g_b = galaxies[b];

            t_distance2 += get_distance(g_a, g_b, &exp_x, &exp_y, 2);
            // t_distance10 += get_distance(g_a, g_b, &exp_x, &exp_y, 10);
            // t_distance100 += get_distance(g_a, g_b, &exp_x, &exp_y, 100);
            t_distance1000000 += get_distance(g_a, g_b, &exp_x, &exp_y, 1000000);
        }
    }

    println!("{t_distance2}");
    // println!("{t_distance10}");
    // println!("{t_distance100}");
    println!("{t_distance1000000}");
}
