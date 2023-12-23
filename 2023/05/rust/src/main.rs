use std::{
    cmp::min,
    io::{self, BufRead, Write},
};

#[derive(Debug)]
struct MapDirective {
    destination: u32,
    source: u32,
    range: u32,
}
#[derive(Debug)]
struct Map {
    _name: String,
    mapping: Vec<MapDirective>,
}

fn main() {
    let mut lines = io::stdin().lock().lines();
    let seeds_input: Vec<u32> = lines
        .next()
        .expect("Error reading line")
        .unwrap()
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|n| n.parse().expect("Failed to parse Integer"))
        .collect();

    lines.next();

    let mut seeds: Vec<u32> = Vec::new();
    for pair in seeds_input.chunks(2) {
        for s in pair[0]..pair[0] + pair[1] {
            seeds.push(s);
        }
    }

    // println!("Seeds: {:?}", seeds);
    println!("Seeds loaded.");
    io::stdout().flush().unwrap();

    let mut maps: Vec<Map> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.expect("Error reading line");

        if line.trim().is_empty() {
            continue;
        }

        if line.chars().nth(0).unwrap().is_alphabetic() {
            let name = line.trim().replace(" map:", "");
            // map_names.push(line.trim().replace(":", ""));
            let mut mapping: Vec<MapDirective> = Vec::new();

            while let Some(map_line) = lines.next() {
                let map_line = map_line.expect("Error reading map line");
                if map_line.is_empty() {
                    break;
                }

                let parsed_ints: Vec<u32> = map_line
                    .split_whitespace()
                    .map(|n| n.parse().expect("Failed to parse map Integer"))
                    .collect();

                mapping.push(MapDirective {
                    destination: parsed_ints[0],
                    source: parsed_ints[1],
                    range: parsed_ints[2],
                });
            }

            maps.push(Map {
                _name: name,
                mapping,
            });
        }
    }

    println!("Maps loaded.");
    io::stdout().flush().unwrap();

    let mut min_final = u32::MAX;

    let mut c_seed = 0usize;
    let m_seed = seeds.len();

    for seed in seeds {
        let mut path: Vec<u32> = Vec::new();
        path.push(seed);

        for map in &maps {
            let source = path.last().unwrap();
            let mut destination: Option<u32> = None;
            for d in &map.mapping {
                if source >= &d.source {
                    let diff = source - d.source;
                    if diff < d.range {
                        destination = Some(d.destination + diff);
                        break;
                    }
                }
            }
            if destination == None {
                destination = Some(source.to_owned())
            }

            path.push(destination.unwrap())
        }

        min_final = min(min_final, *path.last().unwrap());

        c_seed += 1;
        print!(
            "\r{} / {} {:.3}",
            c_seed,
            m_seed,
            c_seed as f64 / m_seed as f64
        );
        io::stdout().flush().unwrap();
    }

    println!("\n{}", min_final);
}
