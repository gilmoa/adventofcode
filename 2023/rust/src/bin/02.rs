use std::{cmp::max, collections::HashMap, io::BufRead};

fn main() {
    let fixed_bag: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut total_games = 0;
    let mut total_power = 0;
    for line in std::io::stdin().lock().lines() {
        let cline = line.unwrap_or(String::from(""));
        let parts: Vec<&str> = cline.split(":").collect();
        assert_eq!(parts.len(), 2);
        let gameid = parts[0].replace("Game ", "").parse::<i32>().unwrap();

        let mut valid_game = true;

        let mut min_bag: HashMap<String, i32> = HashMap::new();

        for set in parts[1].split(";") {
            for cset in set.trim().split(",") {
                let parts: Vec<&str> = cset.trim().split_whitespace().collect();
                let gc: String = parts[1].trim().to_lowercase();
                let gn: i32 = parts[0].parse::<i32>().unwrap();
                if fixed_bag.contains_key(gc.as_str()) {
                    if gn > fixed_bag[gc.as_str()] {
                        valid_game = false;
                    }
                }
                if min_bag.contains_key(gc.as_str()) {
                    min_bag.insert(gc.to_owned(), max(gn, min_bag[gc.as_str()]));
                } else {
                    min_bag.insert(gc.to_owned(), gn);
                }
            }
        }

        if valid_game {
            total_games += gameid
        }

        let mut game_power = 1;
        for (_, v) in &min_bag {
            game_power *= v
        }

        total_power += game_power;
    }

    println!("{}", total_games);
    println!("{}", total_power);
}
