use std::{
    cmp::{max, min},
    fmt::{Display, Formatter, Result},
    io,
};

struct Map<'a> {
    width: usize,
    height: usize,
    grid: &'a mut Vec<char>,
    exp_rows: Vec<usize>,
    exp_cols: Vec<usize>,
}

impl<'a> Map<'a> {
    fn new(width: usize, grid: &'a mut Vec<char>) -> Self {
        let height = grid.len() / width;
        Self {
            width,
            height,
            grid,
            exp_rows: Vec::new(),
            exp_cols: Vec::new(),
        }
    }

    fn get(&self) -> &Vec<char> {
        &self.grid
    }

    // fn get(&self, x: usize, y: usize) -> Option<&char> {
    //     self.grid.get(y * self.width + x)
    // }

    fn calc_expand(&mut self) {
        let empty_row = ".".repeat(self.width);
        for y in 0..self.height {
            let start = y * self.width;
            let slice: String = self
                .grid
                .get(start..start + self.width)
                .expect("Should always find complete line")
                .iter()
                .collect();
            if slice == empty_row {
                self.exp_rows.push(y);
            }
        }

        let empty_col = ".".repeat(self.height);
        for x in 0..self.width {
            let slice: String = self
                .grid
                .iter()
                .enumerate()
                .filter(|(i, _)| i % self.width == x)
                .map(|(_, ch)| ch)
                .collect();
            if slice == empty_col {
                self.exp_cols.push(x)
            }
        }
    }

    fn expand(&mut self) {
        // expand rows
        let mut y: usize = 0;
        while y < self.height {
            let empty_row = ".".repeat(self.width);
            let start = y * self.width;
            let slice: String = self
                .grid
                .get(start..start + self.width)
                .expect("Should always find complete line")
                .iter()
                .collect();
            if slice == empty_row {
                self.grid.splice(start..start, empty_row.chars());
                self.height += 1;
                y += 1
            }
            y += 1;
        }

        // expand cols
        let mut x: usize = 0;
        while x < self.width {
            let empty_col = ".".repeat(self.height);
            let slice: String = self
                .grid
                .iter()
                .enumerate()
                .filter(|(i, _)| i % self.width == x)
                .map(|(_, ch)| ch)
                .collect();
            if slice == empty_col {
                for y in 0..self.height {
                    // +y because everytime we add we shift by one
                    let start = y * self.width + x + y;
                    self.grid.insert(start, '.');
                }
                self.width += 1;
                x += 1;
            }
            x += 1;
        }
    }
}

impl<'a> Display for Map<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut r: String = "".to_string();

        for (i, &ch) in self.grid.iter().enumerate() {
            if i % self.width == 0 {
                r.push('\n');
            }
            r.push(ch);
        }

        write!(
            f,
            "{}\nw{}h{}\nexp_rows: {:?}\nexp_cols: {:?}",
            r.trim(),
            self.width,
            self.height,
            self.exp_rows,
            self.exp_cols
        )
    }
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
    for line in io::stdin().lines() {
        let c_line = line.expect("Should have valid lines");
        width = min(width, c_line.len());
        for c in c_line.trim().chars() {
            grid.push(c);
        }
    }

    let mut map = Map::new(width, &mut grid);

    map.calc_expand();
    // println!("{map}");

    // find galaxies
    let galaxies: Vec<(usize, usize)> = map
        .get()
        .iter()
        .enumerate()
        .filter(|(_, &ch)| ch == '#')
        .map(|(i, _)| (i % map.width, i / map.width))
        .collect();

    // println!("Galaxies: {galaxies:?}");

    // let mut galaxies_pairs: Vec<(usize, usize)> = Vec::new();
    // let mut galaxies_distances: Vec<usize> = Vec::new();
    let mut t_distance2: usize = 0;
    let mut t_distance10: usize = 0;
    let mut t_distance100: usize = 0;
    let mut t_distance1000000: usize = 0;

    for a in 0..galaxies.len() - 1 {
        for b in a + 1..galaxies.len() {
            // galaxies_pairs.push((a, b));
            let g_a = galaxies[a];
            let g_b = galaxies[b];
            // galaxies_distances.push(get_distance(g_a, g_b));
            t_distance2 += get_distance(g_a, g_b, &map.exp_cols, &map.exp_rows, 2);
            t_distance10 += get_distance(g_a, g_b, &map.exp_cols, &map.exp_rows, 10);
            t_distance100 += get_distance(g_a, g_b, &map.exp_cols, &map.exp_rows, 100);
            t_distance1000000 += get_distance(g_a, g_b, &map.exp_cols, &map.exp_rows, 1000000);
        }
    }

    // println!("{}", galaxies_pairs.len());
    // println!("Distances : {galaxies_distances:?}");
    // println!("{}", galaxies_distances.iter().sum::<usize>());
    println!("{t_distance2}");
    println!("{t_distance10}");
    println!("{t_distance100}");
    println!("{t_distance1000000}");
}
