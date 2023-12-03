use std::collections::HashMap;

fn main() {
    let lines = include_str!("../../input.txt").lines();
    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let games = line.split(": ").nth(1).unwrap().split("; ");
        let mut is_valid = true;
        for game in games {
            let mut map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
            for round in game.split(", ") {
                let (amount, color) = round.split_once(" ").unwrap();
                map.insert(color, amount.parse().unwrap());
            }
            if map["red"] > 12 || map["green"] > 13 || map["blue"] > 14 {
                is_valid = false;
            }
        }
        if is_valid {
            sum += i + 1;
        }
    }

    println!("{}", sum);
}
