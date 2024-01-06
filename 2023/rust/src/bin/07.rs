use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: u32,
    s_type: HandType,
    s_values: Vec<u32>,
    j_type: HandType,
    j_values: Vec<u32>,
}

fn get_cards_count(cards: &String, joker: bool) -> String {
    let mut card_count = HashMap::<char, u32>::new();
    for c in cards.chars() {
        card_count.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    if joker && card_count.len() > 1 {
        let mut j_v = 0u32;
        if let Some(j) = card_count.get(&'J') {
            j_v = *j;
            card_count.remove(&'J');
        }
        if let Some((m_i, _)) = card_count.iter().max_by_key(|(_, v)| *v) {
            card_count.entry(*m_i).and_modify(|e| *e += j_v);
        }
    }

    let mut values: Vec<u32> = card_count.values().cloned().collect();
    values.sort();

    let t_str: String = values.iter().map(|v| v.to_string()).collect();

    t_str
}

fn get_hand_type(cards: &String, joker: bool) -> HandType {
    use HandType::*;

    let cards_count = get_cards_count(cards, joker);
    // println!("{} - {:?}", cards, cards_count);

    match cards_count.as_str() {
        "5" => FiveKind,
        "14" => FourKind,
        "23" => FullHouse,
        "113" => ThreeKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        _ => panic!("hand should match one type"),
    }
}

fn card2value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        v => v.to_digit(10).expect("Card should be parsable to value"),
    }
}

impl Hand {
    pub fn new(cards: String, bid: u32) -> Self {
        let s_values = cards.chars().map(|c| card2value(c)).collect();
        let s_type = get_hand_type(&cards, false);

        let j_values: Vec<u32> = cards
            .replace("J", "1")
            .chars()
            .map(|c| card2value(c))
            .collect();

        let j_type = get_hand_type(&cards, true);

        Self {
            cards,
            bid,
            s_type,
            s_values,
            j_type,
            j_values,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.s_type != other.s_type {
            return self.s_type.cmp(&other.s_type);
        } else {
            return self.s_values.cmp(&other.s_values);
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut total = 0u32;
    let mut total_j = 0u32;
    let mut hands = Vec::<Hand>::new();
    for line in io::stdin().lock().lines() {
        let c_line = line.expect("Error reading line");
        let hand = Hand::new(
            c_line
                .trim()
                .split_whitespace()
                .nth(0)
                .expect("Line should always have a space")
                .to_ascii_uppercase(),
            c_line
                .trim()
                .split_whitespace()
                .nth(1)
                .expect("Line should always have a space")
                .parse()
                .expect("Line should always end with a number"),
        );

        hands.push(hand);
    }

    hands.sort();
    // println!("{:?}", hands);

    for (i, h) in hands.iter().enumerate() {
        total += (i as u32 + 1) * h.bid;
    }

    println!("{total}");

    hands.sort_by(|a, b| {
        if a.j_type != b.j_type {
            return a.j_type.cmp(&b.j_type);
        } else {
            return a.j_values.cmp(&b.j_values);
        }
    });

    for (i, h) in hands.iter().enumerate() {
        total_j += (i as u32 + 1) * h.bid;
    }

    println!("{total_j}");
}
