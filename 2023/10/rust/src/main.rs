use std::{
    collections::{HashMap, VecDeque},
    io::{self, BufRead},
};

struct Queue<T> {
    items: VecDeque<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            items: VecDeque::new(),
        }
    }
    pub fn add(&mut self, x: T) {
        self.items.push_back(x);
    }
    pub fn get(&mut self) -> Option<T> {
        self.items.pop_front()
    }
    pub fn is_empty(&mut self) -> bool {
        self.items.len() == 0
    }
}

#[derive(Debug, Clone)]
struct Node {
    pipe: char,
    v: bool,
    d: u32,
    n: Vec<(usize, usize)>,
}

impl Node {
    pub fn new(c: char) -> Self {
        Self {
            pipe: c,
            v: false,
            d: 0,
            n: vec![],
        }
    }
}

fn print_map(map: &HashMap<(usize, usize), Node>) {
    let mut y = 0usize;
    let mut x = 0usize;

    while let Some(n) = map.get(&(x, y)) {
        print!("{}", n.pipe);
        x += 1;
        if map.get(&(x, y)).is_none() {
            println!("");
            x = 0;
            y += 1;
        }
    }
}

fn print_map_distances(map: &HashMap<(usize, usize), Node>) {
    let mut y = 0usize;
    let mut x = 0usize;

    while let Some(n) = map.get(&(x, y)) {
        print!("{}", n.d);
        x += 1;
        if map.get(&(x, y)).is_none() {
            println!("");
            x = 0;
            y += 1;
        }
    }
}

fn visit_node(queue: &mut Queue<(usize, usize, u32)>, map: &mut HashMap<(usize, usize), Node>) {
    let t = queue.get();
    if t.is_none() {
        return;
    }

    let (x, y, d) = t.unwrap();

    if let Some(node) = map.get_mut(&(x, y)) {
        if node.v {
            return;
        }

        node.v = true;
        node.d = d;

        for (x, y) in &node.n {
            queue.add((*x, *y, d + 1));
        }
    }
}

fn main() {
    let mut start_node = (0usize, 0usize);
    let mut map = HashMap::<(usize, usize), Node>::new();
    for (y, line) in io::stdin().lock().lines().enumerate() {
        for (x, c) in line.expect("Error parsing line").char_indices() {
            map.insert((x, y), Node::new(c));
            if c == 'S' {
                start_node = (x, y);
            }
        }
    }

    // println!("{:?}", &map);
    // print_map(&map);

    let mut ns: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for ((x, y), n) in &map {
        let conn = match n.pipe {
            '|' => vec![(0, -1), (0, 1)],
            'L' => vec![(0, -1), (1, 0)],
            'J' => vec![(0, -1), (-1, 0)],
            '-' => vec![(-1, 0), (1, 0)],
            '7' => vec![(-1, 0), (0, 1)],
            'F' => vec![(1, 0), (0, 1)],
            '.' => vec![],
            'S' => vec![(0, -1), (-1, 0), (1, 0), (0, 1)],
            _ => panic!("Pipe should be exaustive"),
        };

        let mut nn: Vec<(usize, usize)> = Vec::new();
        for (x1, y1) in conn {
            let n_x = x.checked_add_signed(x1);
            let n_y = y.checked_add_signed(y1);
            if let (Some(n_x), Some(n_y)) = (n_x, n_y) {
                if map.contains_key(&(n_x, n_y)) {
                    nn.push((n_x, n_y));
                }
            }
        }
        // println!("{:?} has neig: {:?}", (x, y), nn);
        ns.insert((*x, *y), nn);
    }

    for ((x, y), ns) in &ns {
        if let Some(n) = map.get_mut(&(*x, *y)) {
            if ns.len() < 2 {
                n.pipe = '.'
            } else {
                n.n = ns.clone();
            }
        }
    }

    ns.clear();
    for ((x, y), n) in &map {
        let mut nn: Vec<(usize, usize)> = Vec::new();
        for (tx, ty) in &n.n {
            if let Some(tn) = map.get(&(*tx, *ty)) {
                if !tn.n.contains(&(*x, *y)) {
                    nn.push((*tx, *ty));
                }
            }
        }
        ns.insert((*x, *y), nn);
    }

    for ((x, y), ns) in &ns {
        if let Some(n) = map.get_mut(&(*x, *y)) {
            for (tx, ty) in ns {
                if let Some(i) = n.n.iter().position(|(x, y)| tx == x && ty == y) {
                    n.n.remove(i);
                }
            }
            if n.n.len() < 2 {
                n.pipe = '.'
            }
        }
    }

    // println!("{:?}", &map);
    // print_map_distances(&map);

    let mut queue = Queue::<(usize, usize, u32)>::new();
    queue.add((start_node.0, start_node.1, 0));
    while !queue.is_empty() {
        visit_node(&mut queue, &mut map);
    }
    // print_map_distances(&map);

    let max_d = map.values().map(|n| n.d).max();

    println!("{:?}", max_d.unwrap());
}
