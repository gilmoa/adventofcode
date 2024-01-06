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

    let mut total = 0;
    for line in std::io::stdin().lock().lines() {
        let mut cline = line.unwrap_or("".to_string());

        for (l, v) in &litteraltonumber {
            cline = cline.replace(l, v);
        }

        let mut first: Option<i32> = None;
        let mut last: Option<i32> = None;
        for c in cline.trim().chars() {
            if let Ok(i) = c.to_string().parse::<i32>() {
                if first == None {
                    first = Some(i)
                }
                last = Some(i)
            }
        }

        let ltotal = first.unwrap_or(0) * 10 + last.unwrap_or(0);

        total += ltotal;

        println!("{}. {:?}, {:?} = {}", cline, first, last, ltotal)
    }
    println!("{}", total)
}
