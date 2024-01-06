use std::{collections::HashMap, io::BufRead};

fn main() {
    let litteraltonumber: HashMap<&str, &str> = HashMap::from([
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "thr3ee"),
        ("four", "fo4ur"),
        ("five", "fi5ve"),
        ("six", "s6ix"),
        ("seven", "sev7en"),
        ("eight", "eig8ht"),
        ("nine", "ni9ne"),
    ]);

    let mut total1: u32 = 0;
    let mut total2: u32 = 0;

    for line in std::io::stdin().lock().lines() {
        let mut cline = line.unwrap_or("".to_string());

        let mut first = 0;
        let mut last = 0;

        for c in cline.trim().chars() {
            if let Ok(i) = c.to_string().parse::<u32>() {
                if first == 0 {
                    first = i
                }
                last = i
            }
        }

        total1 += first * 10 + last;

        for (l, v) in &litteraltonumber {
            cline = cline.replace(l, v);
        }

        first = 0;

        for c in cline.trim().chars() {
            if let Ok(i) = c.to_string().parse::<u32>() {
                if first == 0 {
                    first = i
                }
                last = i
            }
        }

        total2 += first * 10 + last;
    }

    println!("{}", total1);
    println!("{}", total2);
}
