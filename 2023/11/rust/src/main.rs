use std::{
    cmp::min,
    fmt::{Display, Formatter, Result},
    io,
};

struct Map {
    width: usize,
    grid: Vec<char>,
}

impl Map {
    fn new(width: usize, grid: Vec<char>) -> Self {
        Self { width, grid }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.grid.get(y * self.width + x)
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

        write!(f, "{}", r.trim())
    }
}

fn main() {
    let mut height: usize = 0;
    let mut width: usize = usize::MAX;
    let mut grid: Vec<char> = Vec::new();
    for line in io::stdin().lines() {
        let c_line = line.expect("Should have valid lines");
        height += 1;
        width = min(width, c_line.len());
        for c in c_line.trim().chars() {
            grid.push(c);
        }
    }

    let mut map = Map::new(width, grid);

    println!("{map}");
}
