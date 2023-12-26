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

    let mut steps: u32 = 0;
    let mut c_nodes: Vec<&Node> = nodes
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| v)
        .collect();

    println!("c_nodes: {:?}", c_nodes);

    while steps < u32::MAX {
        let index = steps as usize % instructions.len();
        let dir = instructions
            .get(index)
            .expect("Should loop back and always have a valid index");

        let mut should_stop = true;
        for c_node in c_nodes.iter_mut() {
            let n_node = if *dir == 'L' {
                &c_node.left
            } else {
                &c_node.right
            };

            *c_node = nodes.get(n_node).expect("Nodes should have valid nodes.");

            if !c_node.id.ends_with('Z') {
                should_stop = false
            }
            // println!("{steps} : {dir} - {:?}", c_node.id);
        }

        steps += 1;

        if should_stop {
            break;
        }

        if steps % 1000000 == 0 {
            println!("{steps} / {}", usize::MAX);
        }
    }

    println!("{steps}");
}
