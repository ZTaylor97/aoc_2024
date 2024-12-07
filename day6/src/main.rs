use std::{fs, ops::RemAssign};

#[derive(Debug, Clone, Copy)]
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

    fn is_left(&self, other: &Self) -> bool {
        match other {
            Self::Up => self == &Self::Left,
            Self::Right => self == &Self::Up,
            Self::Down => self == &Self::Right,
            Self::Left => self == &Self::Down,
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
            _ => {
                panic!("shouldn't be here")
            }
        }
    }

    println!("{}", debug_grid(&occupied_map));

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
fn debug_grid(grid: &[Vec<bool>]) -> String {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&cell| if cell { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("\n")
}

fn part_two(input: &str) -> i32 {
    let (mut pos, mut current_direction, map) = init_state(input);

    let mut occupied_map: Vec<Vec<Option<Vec<Direction>>>> =
        vec![vec![None; map[0].len()]; map.len()];
    occupied_map[pos.0 as usize][pos.1 as usize] = Some(vec![Direction::Up]);
    let mut sum = 0;
    'outer: loop {
        let result = pos.clone() + current_direction.direction();
        if result.0 < 0
            || result.1 < 0
            || result.0 >= map.len() as i32
            || result.1 >= map[0].len() as i32
        {
            break 'outer;
        }

        match map[result.0 as usize][result.1 as usize] {
            CellTypes::Empty => {
                if let Some(ref mut stored_directions) =
                    &mut occupied_map[result.0 as usize][result.1 as usize]
                {
                    for stored_direction in stored_directions.iter() {
                        println!(
                            "I'm facing {current_direction:?}. last time, I was facing {stored_direction:?}"
                        );
                        if current_direction.is_left(&stored_direction) {
                            println!("{result:?}");
                            sum += 1;
                            break;
                        }
                    }

                    stored_directions.push(current_direction.clone());
                } else {
                    occupied_map[pos.0 as usize][pos.1 as usize] =
                        Some(vec![current_direction.clone()]);
                }

                pos = result;
            }
            CellTypes::Wall => {
                current_direction.turn();
            }
            _ => {
                panic!("shouldn't be here")
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

    #[test]
    fn test_is_left() {
        let current_dir = Direction::Left;
        let other_dir = Direction::Up;

        assert!(current_dir.is_left(&other_dir));
    }
}
