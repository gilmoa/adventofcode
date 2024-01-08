use core::fmt::Debug;
use std::io::{self, BufRead};

fn is_reflected<T: Clone + Debug + PartialEq>(s: Vec<T>) -> bool {
    let s1: Vec<T> = s.iter().cloned().rev().collect();

    // println!("{:?} ref: {}", s, s == s1);

    s == s1
}

fn get_reflection(map: &Vec<String>) -> usize {
    if map.len() < 2 {
        return 0;
    }

    let h_len = map[0].len() / 2;

    let mut r: usize = 0;
    for x in 1..map[0].len() {
        // println!("+++++++ checking reflection at {x}");

        let mut x_valid = true;
        for s in map {
            let r_len = match h_len < x {
                true => s.len() - x,
                false => x,
            };
            let start = x - r_len;
            let end = x + r_len;
            let s1 = &s[start..end];

            // println!("checking {s1} with {r_len}");
            if !is_reflected(s1.into()) {
                x_valid = false;
                break;
            }
        }
        if x_valid {
            // println!("--- reflection at {x}");
            r += x;
        }
    }

    r
}

fn main() {
    let mut rows: Vec<String> = Vec::new();

    let mut total1: usize = 0;

    let mut counter: usize = 1;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Should always get a line");
        if !line.is_empty() {
            rows.push(line.clone())
        } else {
            let mut cols: Vec<String> = Vec::new();
            for s in &rows {
                for (x, ch) in s.char_indices() {
                    if let Some(t) = cols.get_mut(x) {
                        t.push(ch);
                    } else {
                        cols.insert(x, ch.to_string())
                    }
                }
            }

            println!("======{counter:03}======");
            for x in &rows {
                println!("{counter:03} {x}");
            }
            let ref_r = get_reflection(&rows);
            println!("{counter:03} reflection at ROW {ref_r}");

            let ref_c = get_reflection(&cols);
            println!("{counter:03} reflection at COL {ref_c}");

            let total = (ref_r) + 100 * (ref_c);

            total1 += total;
            println!("=={counter:03} total {total}=");
            rows.clear();
            counter += 1;
        }
    }

    println!("{total1}");
}
