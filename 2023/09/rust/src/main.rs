use std::io::{self, BufRead};

fn get_der(n: &[i32]) -> Vec<i32> {
    let mut r = Vec::<i32>::new();
    for x in 0..n.len() - 1 {
        r.push(n[x + 1] - n[x]);
    }

    r
}

fn main() {
    let mut total: i32 = 0;
    let mut total2: i32 = 0;
    for line in io::stdin().lock().lines() {
        let readings: Vec<i32> = line
            .expect("Error reading line")
            .split_whitespace()
            .map(|v| v.parse::<i32>().expect("Parsing error"))
            .collect();

        let mut der: Vec<Vec<i32>> = Vec::new();
        der.push(get_der(&readings));

        while der.last().unwrap().iter().fold(0, |acc, v| acc + v) != 0 {
            der.push(get_der(der.last().unwrap()))
        }

        // println!("{:?}", der);

        for x in 0..der.len() - 1 {
            let index = der.len() - 1 - x;
            let l_der = der[index].clone().last().unwrap().clone();
            let f_der = der[index].clone().first().unwrap().clone();
            let c_der: &mut Vec<i32> = der[index - 1].as_mut();

            c_der.push(c_der.last().unwrap() + l_der);
            c_der.insert(0, c_der.first().unwrap() - f_der);
        }

        // println!("{:?}", der);

        total += readings.last().unwrap() + der[0].last().unwrap();
        total2 += readings.first().unwrap() - der[0].first().unwrap();
    }

    println!("{total}");
    println!("{total2}");
}
