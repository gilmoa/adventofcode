use core::fmt::Debug;
use std::{
    cmp::max,
    io::{self, BufRead},
};

fn is_reflected<T: Clone + Debug + PartialEq>(s: Vec<T>) -> bool {
    let s1: Vec<T> = s.iter().cloned().rev().collect();

    // println!("{:?} ref: {}", s, s == s1);

    s == s1
}

fn get_reflection(map: &Vec<String>, strict: usize) -> usize {
    if map.len() < 2 {
        return 0;
    }

    let h_len = map[0].len() / 2;

    let mut r: usize = 0;
    for x in 1..map[0].len() {
        // println!("+++++++ checking reflection at {x}");

        let mut x_valid = true;
        let mut smudges = 0;
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
                if strict > 0 && smudges < 2 {
                    // println!("Checking {s1} diff at {x}");
                    // println!("smudges {smudges}");
                    if let Some(d) = diff(
                        s1[..r_len].chars().collect::<Vec<char>>(),
                        s1[r_len..].chars().rev().collect::<Vec<char>>(),
                    ) {
                        if d > strict {
                            x_valid = false;
                            break;
                        } else {
                            smudges += d;
                            if smudges > 1 {
                                x_valid = false;
                                break;
                            }
                        }
                    }
                } else {
                    x_valid = false;
                    break;
                }
            }
        }
        if x_valid {
            // println!("--- reflection at {x}");
            r += x;
        }
    }

    r
}

fn diff<T: PartialEq + Debug>(a: Vec<T>, b: Vec<T>) -> Option<usize> {
    if a.len() != b.len() {
        return None;
    }

    let mut r = 0;
    for x in 0..a.len() {
        if a[x] != b[x] {
            r += 1;
        }
    }

    // println!("DIFF {a:?} and {b:?} = {r:?}");

    match r {
        0 => None,
        x => Some(x),
    }
}

fn main() {
    let mut rows: Vec<String> = Vec::new();

    let mut total1: usize = 0;
    let mut total2: usize = 0;

    let mut _counter: usize = 1;

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

            // println!("======{counter:03}======");
            // for x in &rows {
            //     println!("{counter:03} {x}");
            // }

            //
            // part 1
            //
            let ref_r = get_reflection(&rows, 0);
            // println!("{counter:03} reflection at ROW {ref_r}");

            let ref_c = get_reflection(&cols, 0);
            // println!("{counter:03} reflection at COL {ref_c}");

            let total = (ref_r) + 100 * (ref_c);

            total1 += total;
            // println!("=={counter:03} total {total}=");

            //
            // part 2
            //
            let mut ref_r2 = get_reflection(&rows, 1);
            ref_r2 = max(ref_r2 - ref_r, 0);
            // println!("{counter:03} reflection at ROW {ref_r2}");

            let mut ref_c2 = get_reflection(&cols, 1);
            ref_c2 = max(ref_c2 - ref_c, 0);

            // println!("{counter:03} reflection at COL {ref_c2}");

            let total = (ref_r2) + 100 * (ref_c2);

            total2 += total;
            // println!("=={counter:03} total {total}=");

            rows.clear();
            _counter += 1;
        }
    }

    println!("{total1}");
    println!("{total2}");
}
