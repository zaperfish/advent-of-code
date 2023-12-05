use zf_cc_utils::StrParseExt;

fn main() {
    let lines = include_str!("../../input.txt").lines();

    let mut copies: Vec<i32> = vec![1; lines.clone().count()];
    for (i, line) in lines.enumerate() {
        let (left, right) = line.split_once("|").unwrap();
        let (_, num_left) = left.split_once(":").unwrap();
        let nums_left = num_left.ints();
        let nums_right = right.ints();
        let mut matches = 0;
        for num in nums_left {
            if nums_right.contains(&num) {
                matches += 1;
            }
        }
        for j in 0..matches {
            copies[i + 1 + j] += copies[i];
        }
        dbg!(copies.clone());
    }
    println!("{}", copies.iter().sum::<i32>());
}
