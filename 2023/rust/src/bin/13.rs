use std::{
    cmp::min,
    io::{self, BufRead},
};

struct Map {
    grid: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn to_string(&self) -> String {
        let mut r = "".to_string();

        for (i, ch) in self.grid.iter().enumerate() {
            if i % self.width == 0 {
                r.push('\n');
            }
            r.push(*ch)
        }

        return r.trim().to_string();
    }
}

fn main() {
    let mut buffer = "".to_string();
    for line in io::stdin().lock().lines() {
        let mut line = line.expect("Should always get a line");
        if !line.is_empty() {
            line.push('\n');
            buffer.push_str(&line);
        } else {
            buffer = buffer.trim().to_string();
            // println!("{buffer}");
            if !buffer.is_empty() {
                let mut grid: Vec<char> = Vec::new();
                let mut width: usize = usize::MAX;
                for line in buffer.lines() {
                    width = min(width, line.len());
                    for ch in line.chars() {
                        grid.push(ch);
                    }
                }

                let height = grid.len() / width;
                let map: Map = Map {
                    grid,
                    width,
                    height,
                };

                println!("{}", map.to_string());
            }
            buffer.clear();
        }
    }
}
