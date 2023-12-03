const DICT: [(&str, u32); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn main() {
    let lines = include_str!("../../input.txt").lines();

    let sum: u32 = lines
        .map(|l| {
            let first = (0..l.len()).find_map(|i| {
                DICT.iter()
                    .find_map(|(k, v)| l[i..].starts_with(k).then_some(v))
            });
            let last = (0..l.len()).rev().find_map(|i| {
                DICT.iter()
                    .find_map(|(k, v)| l[i..].starts_with(k).then_some(v))
            });
            first.unwrap() * 10 + last.unwrap()
        })
        .sum();

    println!("{}", sum);
}
