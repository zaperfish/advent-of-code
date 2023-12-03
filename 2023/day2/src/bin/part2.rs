use std::collections::HashMap;

fn main() {
    let lines = include_str!("../../input.txt").lines();
    let mut sum = 0;

    for line in lines {
        let games = line.split(": ").nth(1).unwrap().split("; ");
        let mut map = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for game in games {
            for round in game.split(", ") {
                let (amount, color) = round.split_once(" ").unwrap();
                if amount.parse::<i32>().unwrap() > map[color] {
                    map.insert(color, amount.parse().unwrap());
                }
            }
        }
        sum += map["red"] * map["green"] * map["blue"];
    }

    println!("{}", sum);
}
