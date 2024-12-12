#![feature(iter_array_chunks)]
use std::fs::{self, File};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Error loading file");
    println!("part 1 answer: {}", part_one(contents.as_str()));
    println!("part 2 answer: {}", part_two(contents.as_str()));
}

fn part_one(input: &str) -> u64 {
    let mut i = 0;

    let mut filesystem: Vec<Option<usize>> = Vec::new();
    let mut chunks = input.chars().array_chunks::<2>();

    for [el1, el2] in &mut chunks {
        let files = el1.to_digit(10).unwrap();
        let free_space = el2.to_digit(10).unwrap();

        let mut file_vec = vec![Some(i); files as usize];
        let mut free_vec = vec![None; free_space as usize];

        filesystem.append(&mut file_vec);
        filesystem.append(&mut free_vec);

        i += 1;
    }

    let rem = chunks
        .into_remainder()
        .unwrap()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let mut final_vec = vec![Some(i); rem as usize];
    filesystem.append(&mut final_vec);

    for i in 0..filesystem.len() {
        if let None = filesystem[i] {
            // find first non None value
            for j in (i..filesystem.len()).rev() {
                if let Some(item) = filesystem[j] {
                    filesystem[i] = Some(item);
                    filesystem[j] = None;
                    break;
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for i in 0..filesystem.len() {
        if let Some(item) = filesystem[i] {
            sum += i as u64 * item as u64;
        }
    }

    sum
}

#[derive(Clone, Copy, Debug)]
enum FileType {
    File(usize, u32),
    Empty(u32),
}

fn part_two(input: &str) -> u64 {
    let mut i = 0;

    let mut filesystem: Vec<FileType> = Vec::new();
    let mut chunks = input.chars().array_chunks::<2>();

    for [el1, el2] in &mut chunks {
        let files = el1.to_digit(10).unwrap();
        let free_space = el2.to_digit(10).unwrap();

        if files > 0 {
            filesystem.push(FileType::File(i, files));
        }

        if free_space > 0 {
            filesystem.push(FileType::Empty(free_space));
        }

        i += 1;
    }

    let rem = chunks
        .into_remainder()
        .unwrap()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

    filesystem.push(FileType::File(i, rem));

    let mut filesystem_mod: Vec<FileType> = Vec::new();

    println!("{filesystem:?}");

    // for each number from the end

    for i in (0..filesystem.len()).rev() {
        if let FileType::File(idx, file_size) = filesystem[i] {
            // look for an empty spot starting from the front
            for j in 0..i {
                if let FileType::Empty(empty_size) = filesystem[j] {
                    let diff: i32 = empty_size as i32 - file_size as i32;

                    // if there is space
                    if diff >= 0 {
                        filesystem[j] = filesystem[i];
                        filesystem[i] = 
                    }
                    if diff > 0 {
                        // Insert empty space here

                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 1928)
    }
    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 2858)
    }
}
