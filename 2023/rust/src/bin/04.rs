use std::io::{self, BufRead};

fn main() {
    let mut total: u32 = 0;
    let mut copies = Vec::<u32>::new();
    let mut counter: usize = 0;

    for line in io::stdin().lock().lines() {
        let c_line = line
            .unwrap()
            .trim()
            .split(':')
            .nth(1)
            .unwrap()
            .trim()
            .to_string();
        let wn: Vec<&str> = c_line
            .split('|')
            .nth(0)
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();

        let cn: Vec<&str> = c_line
            .split('|')
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .collect();

        let mut win_count: u32 = 0;
        for n in cn.to_owned() {
            if wn.contains(&n) {
                win_count += 1;
            }
        }

        if win_count > 0 {
            total += 2u32.pow(win_count - 1);
        }

        if let Some(_) = copies.get(counter) {
            copies[counter] += 1;
        } else {
            copies.insert(counter, 1);
        }

        for x in 1usize..win_count as usize + 1 as usize {
            if let Some(_) = copies.get(counter + x) {
                copies[counter + x] += copies[counter]
            } else {
                copies.insert(counter + x, copies[counter])
            }
        }

        // println!("{:?}", copies);
        counter += 1;
    }

    println!("{}", total);
    println!("{}", copies.iter().fold(0, |acc, c| acc + c));
}
