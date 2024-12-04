use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

fn search(grid: &Vec<Vec<char>>, row: usize, col: usize, word: &str) -> u32 {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let word_vec: Vec<char> = word.chars().collect();

    if grid[row][col] != word.chars().next().unwrap() {
        return 0;
    }

    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // Right
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, 0),  // Up
        (1, 1),   // Down-Right
        (1, -1),  // Down-Left
        (-1, 1),  // Up-Right
        (-1, -1), // Up-Left
    ];

    let mut sum = 0;

    for (x_dir, y_dir) in directions {
        let (mut curr_x, mut curr_y) = (row as i32 + x_dir, col as i32 + y_dir);

        let mut k = 1;

        while k < word.len() {
            if curr_x >= width || curr_y >= height || curr_x < 0 || curr_y < 0 {
                break;
            }

            if grid[curr_x as usize][curr_y as usize] != word_vec[k] {
                break;
            }

            curr_x += x_dir;
            curr_y += y_dir;
            k += 1;
        }
        if k == word.len() {
            sum += 1;
        }
    }

    return sum;
}

fn search_two(grid: &Vec<Vec<char>>, row: i32, col: i32, word: &str) -> u32 {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    //bounds check
    if row - 1 < 0 || row + 1 >= width || col - 1 < 0 || col + 1 >= height {
        return 0;
    }

    let word_vec: Vec<char> = word.chars().collect();

    let directions: Vec<(i32, i32)> = vec![
        (-1, -1), // Up-Left
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
        (1, 1),   // Down-Right
    ];

    println!(
        "{},.,{}",
        grid[row as usize - 1][col as usize - 1],
        grid[row as usize + 1][col as usize]
    );
    println!(
        "{},{},{}",
        grid[row as usize - 1][col as usize],
        grid[row as usize][col as usize],
        grid[row as usize + 1][col as usize]
    );
    println!(
        "{},.,{}",
        grid[row as usize + 1][col as usize - 1],
        grid[row as usize + 1][col as usize + 1]
    );

    let cases = vec!["MMSS", "MSMS", "SSMM", "SMSM"];

    for case in cases {
        let result = directions
            .iter()
            .map(|(y, x)| grid[(row + *x) as usize][(row + *y) as usize])
            .collect::<String>();

        print!("{result}\n");

        if result == case {
            println!(
                "{},{},{}",
                grid[row as usize - 1][col as usize - 1],
                grid[row as usize - 1][col as usize],
                grid[row as usize - 1][col as usize + 1]
            );
            println!(
                "{},{},{}",
                grid[row as usize][col as usize - 1],
                grid[row as usize][col as usize],
                grid[row as usize][col as usize + 1]
            );
            println!(
                "{},{},{}",
                grid[row as usize + 1][col as usize - 1],
                grid[row as usize + 1][col as usize],
                grid[row as usize + 1][col as usize + 1]
            );
            println!("This one worked!");
            return 1;
        }
    }

    return 0;
}

fn part_one(input: &str) -> i32 {
    let word = "XMAS";
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("{grid:?}");

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'X' {
                sum += search(&grid, i, j, word);
            }
        }
    }

    sum as i32
}
fn part_two(input: &str) -> i32 {
    let word = "XMAS";
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("{grid:?}");

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 'A' {
                sum += search_two(&grid, i as i32, j as i32, word);
            }
        }
    }

    sum as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 18)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 9)
    }
}
