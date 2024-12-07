use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

#[derive(Debug)]
struct Equation {
    answer: u64,
    operands: Vec<u64>,
}

fn part_one(input: &str) -> u64 {
    let equations = parse_input(input);

    equations.iter().map(|eq| {
        let combos = compute_all_combinations(&eq.operands, Part::One);
        if let Some(_) = combos.get(&eq.answer) {
            eq.answer
        } else {
            0
        }
    }).sum()
}

fn parse_input(input: &str) -> Vec<Equation> {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut split = line.split(':');
            let answer = split.next().unwrap().parse::<u64>().unwrap();
            let operands = split
                .next()
                .unwrap()
                .split(' ')
                .map(|ops_split| ops_split.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            Equation { answer, operands }
        })
        .collect();
    equations
}

enum Part {
    One,
    Two,
}

fn compute_all_combinations(nums: &[u64], part: Part) -> HashSet<u64> {
    let mut results = HashSet::new();

    match part {
        Part::One => compute_combinations_recursive_one(&nums[1..], nums[0], &mut results),
        Part::Two => compute_combinations_recursive_two(&nums[1..], nums[0], &mut results),
    }
    results
}
fn compute_combinations_recursive_one(nums: &[u64], current: u64, results: &mut HashSet<u64>) {
    // base case
    if nums.is_empty() {
        results.insert(current);
        return;
    }
    let next = nums[0];
    let rest = &nums[1..];

    compute_combinations_recursive_one(rest, current + next, results);
    compute_combinations_recursive_one(rest, current * next, results);
}
fn compute_combinations_recursive_two(nums: &[u64], current: u64, results: &mut HashSet<u64>) {
    // base case
    if nums.is_empty() {
        results.insert(current);
        return;
    }

    let next = nums[0];
    let rest = &nums[1..];

    compute_combinations_recursive_two(rest, current + next, results);
    compute_combinations_recursive_two(rest, current * next, results);
    let str = current.to_string() + next.to_string().as_str();
    compute_combinations_recursive_two(rest, str.parse().unwrap(), results);
}

fn part_two(input: &str) -> u64 {
    let equations = parse_input(input);

    equations.iter().map(|eq| {
        let combos = compute_all_combinations(&eq.operands, Part::Two);
        if let Some(_) = combos.get(&eq.answer) {
            eq.answer
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "190:10 19
3267:81 40 27
83:17 5
156:15 6
7290:6 8 6 15
161011:16 10 13
192:17 8 14
21037:9 7 18 13
292:11 6 16 20";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 3749)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 11387)
    }
}
