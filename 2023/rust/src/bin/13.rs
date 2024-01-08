use std::io::{self, BufRead};

fn is_reflected(s: &str) -> bool {
    let s1: Vec<char> = s.chars().collect();
    let s2: Vec<char> = s1.iter().cloned().rev().collect();

    println!("{s} ref: {}", s1 == s2);

    s1 == s2
}

fn get_reflection(s: &str) -> usize {
    let h_len = s.len() / 2;
    if h_len < 1 {
        return 0;
    }
    for x in 1..s.len() - 1 {
        println!("{s} reflecting at {x}");
        let r_len = match h_len < x {
            true => s.len() - x,
            false => x,
        };
        let start = x - r_len;
        let end = x + r_len;
        let s1 = &s[start..end];
        println!("reflected {s1} with {r_len}");

        if is_reflected(s1) {
            return x;
        }
    }

    0
}

fn main() {
    let mut row = "".to_string();
    let mut col = "".to_string();

    let mut total1: usize = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("Should always get a line");
        if !line.is_empty() {
            row = line.clone();
            col.push_str(&line[..1]);
        } else {
            // check row

            let ref_r = get_reflection(&row);
            let ref_c = match ref_r > 0 {
                true => 0,
                false => get_reflection(&col),
            };
            let total = (ref_r) + 100 * (ref_c);

            total1 += total;
            println!("reflection at {ref_r}");
            println!("reflection at {ref_c}");
            println!("total at {total}");
            col.clear();
        }
    }

    let ref_r = get_reflection(&row);
    let ref_c = get_reflection(&col);
    let total = (ref_r) + 100 * (ref_c);

    total1 += total;

    println!("reflection at {ref_r}");
    println!("reflection at {ref_c}");

    println!("{total1}");
}
