use std::fs;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

fn part_one(input: &str) -> i32 {
    let regex = r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)";
    let re = Regex::new(&regex).expect("invalid regex");
    let mut sum = 0;
    for mtch in re.captures_iter(input) {
        let first: i32 = mtch["first"].parse().unwrap();
        let second: i32 = mtch["second"].parse().unwrap();
        sum += first * second;
    }
    sum
}
fn part_two(input: &str) -> i32 {
    let regex = r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)";
    let re = Regex::new(&regex).expect("invalid regex");
    let mut sum = 0;
    
    for mtch in re.captures_iter(input) {
        let first: i32 = mtch["first"].parse().unwrap();
        let second: i32 = mtch["second"].parse().unwrap();
        sum += first * second;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 161)
    }
}
