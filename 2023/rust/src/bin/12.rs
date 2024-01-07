use std::{
    collections::HashMap,
    io::{self, BufRead},
    iter,
};

fn get_state_hash(s: &str, g: &Vec<usize>) -> String {
    format!("{s}-{:?}", g)
}

fn count_valid_permutations(s: String, g: Vec<usize>, c: &mut HashMap<String, usize>) -> usize {
    let hash = get_state_hash(s.as_str(), &g);

    if let Some(x) = c.get(&hash) {
        return *x;
    }

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

    if s.len() < g.len() {
        return 0;
    }

    if s.len() < g.clone().into_iter().sum::<usize>() {
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
    let mut r: usize = 0;

    if c_ch == "?" {
        let a = count_valid_permutations(".".to_string() + &s[1..], g.clone(), c);
        let b = count_valid_permutations("#".to_string() + &s[1..], g.clone(), c);
        r = a + b;
    } else if c_ch == "#" {
        if is_group && &s[c_g..c_g + 1] != "#" {
            r = count_valid_permutations(s[c_g + 1..].to_string(), g[1..].to_vec(), c);
        }
    } else {
        // c_ch == "."
        r = count_valid_permutations(s[1..].to_string(), g.clone(), c);
    }

    c.insert(hash, r);
    return r;
}

fn main() {
    let mut t_permutations1: usize = 0;
    let mut t_permutations2: usize = 0;

    let mut cache: HashMap<String, usize> = HashMap::new();

    for (_i, line) in io::stdin().lock().lines().enumerate() {
        let line = line.expect("Should always parse a line");
        let mut s_line = line.split_whitespace();
        let springs1 = s_line
            .next()
            .expect("Line should always split in two by whitespace")
            .to_string();

        let groups1: Vec<usize> = s_line
            .next()
            .expect("Line should always split in two by whitespace")
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        let springs2 = iter::repeat(springs1.clone())
            .take(5)
            .map(String::from)
            .collect::<Vec<String>>()
            .join("?");

        let groups2 = groups1.repeat(5);

        let c_permutations1 = count_valid_permutations(springs1, groups1, &mut cache);
        t_permutations1 += c_permutations1;

        let c_permutations2 = count_valid_permutations(springs2, groups2, &mut cache);
        t_permutations2 += c_permutations2;

        // println!("{_i} = {c_permutations1}, {c_permutations2}");
    }

    println!("{t_permutations1}");
    println!("{t_permutations2}");
}
