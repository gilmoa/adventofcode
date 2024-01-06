use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

fn gdc(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gdc(b, a % b)
}

fn lcm(n: &[usize]) -> usize {
    if n.len() == 1 {
        return n[0];
    }
    let a = n[0];
    let b = lcm(&n[1..]);
    a * b / gdc(a, b)
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let instructions: Vec<char> = lines
        .next()
        .expect("Cannot read line")
        .expect("Cannot read line")
        .chars()
        .collect();

    lines.next();

    let mut nodes = HashMap::<String, Node>::new();
    for line in lines {
        let c_line = line
            .expect("Cannot read line")
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .replace(",", "");

        let parts: Vec<String> = c_line
            .split_whitespace()
            .map(|s| s.to_ascii_uppercase())
            .collect();

        let id = parts.get(0).expect("Should have 1 field").to_owned();
        let left = parts.get(1).expect("Should have 2 field").to_owned();
        let right = parts.get(2).expect("Should have 3 field").to_owned();

        nodes.insert(id.clone(), Node { id, left, right });
    }

    let mut steps: usize = 0;
    let mut c_nodes: Vec<&Node> = nodes
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| v)
        .collect();

    let mut c_steps = vec![0usize; c_nodes.len()];
    // println!("c_nodes: {:?}", c_nodes);

    while steps < usize::MAX {
        let index = steps % instructions.len();
        let dir = instructions
            .get(index)
            .expect("Should loop back and always have a valid index");

        steps += 1;
        for (i, c_node) in c_nodes.iter_mut().enumerate() {
            let n_node = if *dir == 'L' {
                &c_node.left
            } else {
                &c_node.right
            };

            *c_node = nodes.get(n_node).expect("Nodes should have valid nodes.");

            if c_node.id.ends_with('Z') && c_steps[i] == 0 {
                c_steps[i] = steps;
                // println!("{:?}", c_steps);
            }
        }

        if c_steps
            .iter()
            .map(|v| if *v > 0 { 1 } else { 0 })
            .product::<usize>()
            != 0
        {
            break;
        }
    }

    println!("{:?}", lcm(&c_steps));
    // println!("{steps}");
}
