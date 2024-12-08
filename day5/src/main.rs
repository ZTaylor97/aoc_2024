use std::collections::BTreeMap;
use std::fs;

fn main() {
    let rules_contents = fs::read_to_string("rules.txt").expect("Error loading file");
    let pages_contents = fs::read_to_string("pages.txt").expect("Error loading file");
    println!(
        "part 1 answer: {}",
        part_one(rules_contents.as_str(), pages_contents.as_str())
    );
    println!(
        "part 2 answer: {}",
        part_two(rules_contents.as_str(), pages_contents.as_str())
    );
}

fn get_rules(raw_rules: &str) -> BTreeMap<i32, Vec<i32>> {
    let mut rules_map: BTreeMap<i32, Vec<i32>> = BTreeMap::new();
    for line in raw_rules.lines() {
        let pair_vec = line
            .split('|')
            .map(|split| split.parse().unwrap())
            .collect::<Vec<i32>>();

        rules_map.entry(pair_vec[0]).or_default().push(pair_vec[1]);
    }

    rules_map
}

fn get_pages(raw_pages: &str) -> Vec<Vec<i32>> {
    raw_pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|split| split.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_one(raw_rules: &str, raw_pages: &str) -> i32 {
    let pages = get_pages(raw_pages);
    let rules_map = get_rules(raw_rules);

    let mut sum = 0;
    for page in pages {
        if let Some(result) = search_one(&page[..], &rules_map) {
            sum += result;
        }
    }

    sum
}

fn search_one(page: &[i32], rules: &BTreeMap<i32, Vec<i32>>) -> Option<i32> {
    for i in 0..page.len() {
        // check every preceding element
        for j in 0..i {
            if let Some(rule) = rules.get(&(page[i] as i32)) {
                if rule.contains(&page[j]) {
                    return None;
                }
            }
        }
    }

    return Some(page[(page.len() as f32 / 2.0_f32).floor() as usize]);
}

fn search_two(mut page: Vec<i32>, rules: &BTreeMap<i32, Vec<i32>>) -> Option<i32> {
    let mut modified = false;
    'outer: loop {
        // Stop looping when this flag is not set
        let mut incorrect = false;
        // check every preceding element
        for i in 0..page.len() {
            for j in 0..i {
                if let Some(rule) = rules.get(&(page[i] as i32)) {
                    if rule.contains(&page[j]) {
                        incorrect = true;
                        modified = true;
                        let temp_element = page[i];
                        let temp_incorrect = page[j];

                        page[j] = temp_element;
                        page[i] = temp_incorrect;
                        break;
                    }
                }
            }
        }

        if !incorrect {
            break 'outer;
        }
    }

    if !modified {
        return None;
    } else {
        Some(page[(page.len() as f32 / 2.0_f32).floor() as usize])
    }
}

fn part_two(raw_rules: &str, raw_pages: &str) -> i32 {
    let pages = get_pages(raw_pages);
    let rules_map = get_rules(raw_rules);

    let mut sum = 0;
    for page in pages {
        if let Some(result) = search_two(page, &rules_map) {
            sum += result;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_RULES: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
    const TEST_PAGES: &str = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_RULES, TEST_PAGES), 143)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_RULES, TEST_PAGES), 123)
    }
}
