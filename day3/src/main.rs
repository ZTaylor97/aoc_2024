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
    re.captures_iter(input)
        .map(|mtch| mtch["first"].parse::<i32>().unwrap() * mtch["second"].parse::<i32>().unwrap())
        .sum()
}
fn part_two(input: &str) -> i32 {
    let regex = r"(don't|do)|mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)";
    let re = Regex::new(&regex).expect("invalid regex");
    let mut sum = 0;
    let mut enable = true;
    for mtch in re.captures_iter(input) {
        if let Some(matched) = mtch.get(1) {
            println!("found {}", matched.as_str());
            match matched.as_str() {
                "don't" => enable = false,
                "do" => enable = true,
                _ => unreachable!("You fucked up"),
            }
        } else if let (Some(first), Some(second)) = (mtch.name("first"), mtch.name("second")) {
            if enable {
                sum +=
                    first.as_str().parse::<i32>().unwrap() * second.as_str().parse::<i32>().unwrap()
            }
        }
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
