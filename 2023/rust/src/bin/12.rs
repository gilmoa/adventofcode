use std::{
    io::{self, BufRead},
    time::{SystemTime, UNIX_EPOCH},
};

fn reduce_line(l: &String) -> String {
    let mut r = "".to_string();
    let mut p_ch = '.';
    for c in l.chars() {
        if p_ch != '.' || c != '.' {
            r.push(c);
        }
        p_ch = c
    }

    r
}

fn count_valid_permutations(s: String, g: Vec<usize>) -> usize {
    if s.len() < 1 {
        return match g.len() < 1 {
            true => 1,
            false => 0,
        };
    }

    if g.len() < 1 {
        return match s.contains("#") {
            true => 0,
            false => 1,
        };
    }

    // println!("{s}, {:?}", g);
    if s.len() < g.len() {
        return 0;
    }

    if s.len() < g.clone().into_iter().sum::<usize>() {
        return 0;
    }

    if s.len() < g.clone().into_iter().sum::<usize>() - g.len() {
        return 0;
    }

    let c_g = g[0];
    let is_group = !s[..c_g].to_string().contains(".");

    if c_g == s.len() {
        return match is_group {
            true => 1,
            false => 0,
        };
    }

    let c_ch = &s[..1];

    if c_ch == "." {
        return count_valid_permutations(s[1..].to_string(), g.clone());
    }

    if c_ch == "?" {
        let a = count_valid_permutations(".".to_string() + &s[1..], g.clone());
        let b = count_valid_permutations("#".to_string() + &s[1..], g.clone());
        return a + b;
    }

    if c_ch == "#" {
        if is_group && &s[c_g..c_g + 1] != "#" {
            return count_valid_permutations(s[c_g + 1..].to_string(), g[1..].to_vec());
        } else {
            return 0;
        }
    }

    count_valid_permutations(s[1..].to_string(), g.clone())
}

fn main() {
    let mut t_permutations: usize = 0;

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    for (i, line) in io::stdin().lock().lines().enumerate() {
        let line = line.expect("Should always parse a line");
        let mut s_line = line.split_whitespace();
        let springs = s_line
            .next()
            .expect("Line should always split in two by whitespace")
            .to_string()
            + ".";

        let groups: Vec<usize> = s_line
            .next()
            .expect("Line should always split in two by whitespace")
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let c_permutations = count_valid_permutations(springs, groups);
        println!("{i} = {c_permutations}");
        t_permutations += c_permutations;
    }

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    dbg!(end - start);

    println!("{t_permutations}");
}
