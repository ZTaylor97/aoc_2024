use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

#[derive(Debug)]
struct Equation {
    answer: u64,
    operands: Vec<u32>,
}

fn part_one(input: &str) -> i32 {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let mut split = line.split(':');
            let answer = split.next().unwrap().parse::<u64>().unwrap();
            let operands = split
                .next()
                .unwrap()
                .split(' ')
                .map(|ops_split| ops_split.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            Equation { answer, operands }
        })
        .collect();

    for equation in equations {
        
    }

    0
}

fn part_two(input: &str) -> i32 {
    0
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
        assert_eq!(part_two(TEST_INPUT), 6)
    }
}
