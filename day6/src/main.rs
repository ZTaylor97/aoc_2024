use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellTypes {
    Empty,
    Wall,
}

impl Default for CellTypes {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct Point(i32, i32);

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Direction {
    fn direction(&self) -> Point {
        match self {
            Self::Up => Point(-1, 0),
            Self::Right => Point(0, 1),
            Self::Down => Point(1, 0),
            Self::Left => Point(0, -1),
        }
    }
    fn turn(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

fn part_one(input: &str) -> i32 {
    let (mut pos, mut direction, map) = init_state(input);

    let mut occupied_map: Vec<Vec<bool>> = vec![vec![false; map[0].len()]; map.len()];
    occupied_map[pos.0 as usize][pos.1 as usize] = true;

    'outer: loop {
        let result = pos.clone() + direction.direction();
        if result.0 < 0
            || result.1 < 0
            || result.0 >= map.len() as i32
            || result.1 >= map[0].len() as i32
        {
            break 'outer;
        }

        match map[result.0 as usize][result.1 as usize] {
            CellTypes::Empty => {
                occupied_map[result.0 as usize][result.1 as usize] = true;
                pos = result;
            }
            CellTypes::Wall => direction.turn(),
        }
    }

    occupied_map
        .iter()
        .flat_map(|inner| inner.iter())
        .filter(|&&value| value)
        .count()
        .try_into()
        .unwrap()
}

fn init_state(input: &str) -> (Point, Direction, Vec<Vec<CellTypes>>) {
    let mut pos: Point = Point(0, 0);
    let direction = Direction::Up;
    let map: Vec<Vec<CellTypes>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .map(|(col, ch)| match ch {
                    '.' => CellTypes::Empty,
                    '#' => CellTypes::Wall,
                    '^' => {
                        pos = Point(row as i32, col as i32);
                        CellTypes::Empty
                    }
                    _ => panic!("shouldn't be here"),
                })
                .collect::<Vec<CellTypes>>()
        })
        .collect();
    (pos, direction, map)
}
fn _debug_grid(grid: &[Vec<bool>]) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell { '$' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn part_two(input: &str) -> i32 {
    let (initital_pos, initial_direction, map) = init_state(input);

    let limit = 4 * map.len() * map[0].len();
    let mut sum = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let mut direction = initial_direction.clone();
            let mut pos = initital_pos.clone();
            let mut num_moves = 0;
            let mut new_map = map.clone();
            // Skip this if map is already wall
            if new_map[i][j] == CellTypes::Wall {
                continue;
            }
            new_map[i][j] = CellTypes::Wall;

            'outer: loop {
                // loop detected
                if num_moves >= limit {
                    sum += 1;
                    break;
                }

                let result = pos.clone() + direction.direction();
                if result.0 < 0
                    || result.1 < 0
                    || result.0 >= map.len() as i32
                    || result.1 >= map[0].len() as i32
                {
                    break 'outer;
                }

                match new_map[result.0 as usize][result.1 as usize] {
                    CellTypes::Empty => {
                        num_moves += 1;
                        pos = result;
                    }
                    CellTypes::Wall => direction.turn(),
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 41)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 6)
    }
}
