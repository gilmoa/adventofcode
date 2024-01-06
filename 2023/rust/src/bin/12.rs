use std::io;

fn line_from_groups(group: &Vec<usize>) -> String {
    group
        .iter()
        .map(|&x| "#".repeat(x))
        .collect::<Vec<String>>()
        .join(".")
}

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

fn perm(s: String, k: &Vec<char>, l: usize, t: usize, r: &mut Vec<String>) {
    if l < t {
        for &ch in k {
            perm(s.clone() + ch.to_string().as_str(), k, l + 1, t, r);
        }
    } else {
        r.push(s)
    }
}

fn get_permutations(l: &String) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    let positions: Vec<usize> = l
        .char_indices()
        .filter(|(_, ch)| *ch == '?')
        .map(|(i, _)| i)
        .collect();

    let mut permutations: Vec<String> = Vec::new();
    perm(
        "".to_string(),
        &vec!['.', '#'],
        0,
        positions.len(),
        &mut permutations,
    );

    for perm in permutations {
        let mut s = l.clone();
        for (ch, i) in perm.chars().zip(positions.clone()) {
            // println!("Replacing {ch} at {i}");
            s.replace_range(i..i + 1, ch.to_string().as_str());
        }
        r.push(s);
    }

    r
}

fn main() {
    let mut t_permutations: usize = 0;
    for line in io::stdin().lines() {
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

        // groups.repeat(5);

        let group_line = line_from_groups(&groups) + ".";
        let permutations = get_permutations(&springs);
        // println!("{:?}", &group_line);
        // println!("{:?}", &permutations);

        // let mut tmp = 0usize;
        for perm in permutations {
            // println!("{}", reduce_line(&perm));
            if reduce_line(&perm) == group_line {
                // tmp += 1;
                t_permutations += 1;
            }
        }

        // println!("{}", tmp);
    }

    println!("{t_permutations}");
}
