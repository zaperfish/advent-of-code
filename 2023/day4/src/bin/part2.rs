use zf_cc_utils::*;

fn main() {
    let lines = include_str!("../../input.txt").lines();
    let mut cards = vec![1; lines.clone().count()];

    for (i, line) in lines.enumerate() {
        let (wins, my) = line.split(":").nth(1).unwrap().split_once("|").unwrap();
        let matches = wins
            .ints()
            .iter()
            .fold(0, |acc, n| acc + my.ints().contains(n) as usize);
        for j in 0..matches {
            cards[i + 1 + j] += cards[i];
        }
    }

    println!("{}", cards.iter().sum::<i32>());
}
