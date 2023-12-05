use zf_cc_utils::*;

fn main() {
    let mut sum = 0;

    for line in include_str!("../../input.txt").lines() {
        let (wins, my) = line.split(":").nth(1).unwrap().split_once("|").unwrap();
        let matches = wins
            .ints()
            .iter()
            .fold(0, |acc, n| acc + my.ints().contains(n) as u32);
        if matches > 0 {
            sum += 2u32.pow(matches - 1);
        }
    }

    println!("{}", sum);
}
