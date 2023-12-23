use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines();
    let times: Vec<u64> = lines
        .next()
        .expect("Error reading line")
        .unwrap()
        .replace("Time:", "")
        .trim()
        .split_whitespace()
        .map(|n| n.parse().expect("Error parsing integers"))
        .collect();

    let distances: Vec<u64> = lines
        .next()
        .expect("Error reading line")
        .unwrap()
        .replace("Distance:", "")
        .trim()
        .split_whitespace()
        .map(|n| n.parse().expect("Error parsing integers"))
        .collect();

    let mut total: u64 = 1;
    for (i, t) in times.iter().enumerate() {
        let distance = distances
            .get(i)
            .expect("Missmatch with Times and Distances count.");

        let mut count: u64 = 0;
        for x in 1..*t {
            let c_d = x * (t - x);
            if c_d > *distance {
                count += 1;
            }
        }

        // println!("{t}-{distance}: {count}");

        if count > 0 {
            total *= count
        }
    }

    let time: u64 = times
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("Failed to parse time integer");

    let distance: u64 = distances
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .expect("Failed to parse distance integer");

    let mut count: u64 = 0;
    for x in 1..time {
        let c_d = x * (time - x);
        if c_d > distance {
            count += 1;
        }
    }

    println!("{:?}", total);
    println!("{:?}", count);
}
