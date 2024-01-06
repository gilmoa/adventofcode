use std::{
    cmp::min,
    fmt::{Display, Formatter, Result},
    io,
};

struct Map {
    width: usize,
    height: usize,
    grid: Vec<char>,
}

impl Map {
    fn new(width: usize, grid: Vec<char>) -> Self {
        let height = grid.len() / width;
        Self {
            width,
            height,
            grid,
        }
    }

    fn get(&self) -> &Vec<char> {
        &self.grid
    }

    // fn get(&self, x: usize, y: usize) -> Option<&char> {
    //     self.grid.get(y * self.width + x)
    // }

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

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut r: String = "".to_string();

        for (i, &ch) in self.grid.iter().enumerate() {
            if i % self.width == 0 {
                r.push('\n');
            }
            r.push(ch);
        }

        write!(f, "{}\nw{}h{}", r.trim(), self.width, self.height)
    }
}

fn get_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let d_x = a.0.abs_diff(b.0);
    let d_y = a.1.abs_diff(b.1);
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

    let mut map = Map::new(width, grid);

    // println!("{map}");
    map.expand();
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
    let mut galaxies_distances: Vec<usize> = Vec::new();
    for a in 0..galaxies.len() - 1 {
        for b in a + 1..galaxies.len() {
            // galaxies_pairs.push((a, b));
            let g_a = galaxies[a];
            let g_b = galaxies[b];
            galaxies_distances.push(get_distance(g_a, g_b));
        }
    }

    // println!("{}", galaxies_pairs.len());
    // println!("Distances : {galaxies_distances:?}");
    println!("{}", galaxies_distances.iter().sum::<usize>());
}
