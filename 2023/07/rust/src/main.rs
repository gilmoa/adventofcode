#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    s_type: Option<u32>,
}

fn main() {
    println!("Hello, world!");
    let mut t = vec![1, 7, 9, 2];
    println!("{:?}", t);
    t.sort();
    println!("{:?}", t);
}
