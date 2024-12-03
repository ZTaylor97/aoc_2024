use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");

    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

fn part_one(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let levels = line
            .split(' ')
            .map(|s| s.parse().expect("Error unwrapping i32"))
            .collect::<Vec<i32>>();

        let is_descending = levels.windows(2).all(|w| w[0] > w[1]);
        let is_ascending = levels.windows(2).all(|w| w[1] > w[0]);
        let is_safe = levels.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);

        if (is_ascending || is_descending) && is_safe {
            println!("{} is safe", line);
            sum += 1;
        } else {
            println!("{} is not safe", line);
        }
    }
    sum
}
fn part_two(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let levels = line
            .split(' ')
            .map(|s| s.parse().expect("Error unwrapping i32"))
            .collect::<Vec<i32>>();

        for i in 0..levels.len() {
            // yes I know this is n^2 but I don't care
            let filtered_levels: Vec<_> = levels
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != i)
                .map(|(_, &val)| val)
                .collect();

            let is_descending = filtered_levels.windows(2).all(|w| w[0] > w[1]);
            let is_ascending = filtered_levels.windows(2).all(|w| w[1] > w[0]);
            let is_safe = filtered_levels.windows(2).all(|w| (w[0] - w[1]).abs() <= 3);

            if (is_ascending || is_descending) && is_safe {
                sum += 1;
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4)
    }
}
