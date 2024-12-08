use std::{
    collections::{BTreeMap, HashSet},
    fs,
};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

fn parse_input(input: &str) -> (BTreeMap<char, Vec<(i32, i32)>>, (u32, u32)) {
    let mut map: BTreeMap<char, Vec<(i32, i32)>> = BTreeMap::new();
    let mut bounds = (0, 0);

    bounds.0 = input.lines().count() as u32;
    bounds.1 = input.lines().next().unwrap().len() as u32;

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.char_indices() {
            if ch != '.' {
                map.entry(ch).or_default().push((row as i32, col as i32));
            }
        }
    }

    (map, bounds)
}

fn part_one(input: &str) -> u64 {
    let (towers, bounds) = parse_input(input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for vec in towers.values() {
        for (i, (row, col)) in vec.iter().enumerate() {
            for j in (i + 1)..vec.len() {
                let (other_row, other_col) = vec[j];

                let (diff_row, diff_col) = ((row - other_row).abs(), (col - other_col).abs());

                let node_1 = if *col < other_col {
                    (row - diff_row, col - diff_col)
                } else {
                    (row - diff_row, col + diff_col)
                };

                let node_2 = if *col < other_col {
                    (other_row + diff_row, other_col + diff_col)
                } else {
                    (other_row + diff_row, other_col - diff_col)
                };

                if is_node_in_bounds(node_1, bounds) {
                    antinodes.insert(node_1);
                }

                if is_node_in_bounds(node_2, bounds) {
                    antinodes.insert(node_2);
                }
            }
        }
    }

    antinodes.len() as u64
}

fn part_two(input: &str) -> u64 {
    let (towers, bounds) = parse_input(input);

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for vec in towers.values() {
        for (i, (row, col)) in vec.iter().enumerate() {
            antinodes.insert((*row, *col));

            for j in (i + 1)..vec.len() {
                let (other_row, other_col) = vec[j];

                let (diff_row, diff_col) = ((row - other_row).abs(), (col - other_col).abs());

                let mut node_1 = if *col < other_col {
                    (row - diff_row, col - diff_col)
                } else {
                    (row - diff_row, col + diff_col)
                };
                while is_node_in_bounds(node_1, bounds) {
                    antinodes.insert(node_1);
                    node_1 = if *col < other_col {
                        (node_1.0 - diff_row, node_1.1 - diff_col)
                    } else {
                        (node_1.0 - diff_row, node_1.1 + diff_col)
                    };
                }

                let mut node_2 = if *col < other_col {
                    (other_row + diff_row, other_col + diff_col)
                } else {
                    (other_row + diff_row, other_col - diff_col)
                };

                while is_node_in_bounds(node_2, bounds) {
                    antinodes.insert(node_2);
                    node_2 = if *col < other_col {
                        (node_2.0 + diff_row, node_2.1 + diff_col)
                    } else {
                        (node_2.0 + diff_row, node_2.1 - diff_col)
                    };
                }
            }
        }
    }

    print_grid(bounds.0 as i32, bounds.1 as i32, &antinodes);

    antinodes.len() as u64
}

fn is_node_in_bounds(node: (i32, i32), bounds: (u32, u32)) -> bool {
    !(node.0 < 0 || node.1 < 0 || node.0 >= bounds.0 as i32 || node.1 >= bounds.1 as i32)
}

fn print_grid(rows: i32, cols: i32, filled_cells: &HashSet<(i32, i32)>) {
    for row in 0..rows {
        for col in 0..cols {
            if filled_cells.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!(); // Move to the next row
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 14)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 34)
    }
}
