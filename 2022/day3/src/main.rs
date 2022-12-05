// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
// 97 = a  char - 96
// 122 = z
// 65 = A  char - 64 + 26 == char - 38
// 90 = Z

use std::collections::HashSet;

/*
// pretty clever way to create a HashMap of letters to scores
// not really necessary here
// https://www.youtube.com/watch?v=yBJJYkC5cdk
let letter_scores = ('a'..='z')
    .chain('A'..='Z')
    .enumerate()
    .map(|(idx, c)| (c, idx + 1))
    .collect::<HashMap<char, usize>>();
*/

fn char_to_value(c: u8) -> u32 {
    (if c < 91 { c - 38 } else { c - 96 }) as u32
}

fn part_1(input: String) -> u32 {
    let mut common_letters: Vec<char> = Vec::new();
    for line in input.lines() {
        let mid = line.len() / 2;
        let left = &line[..mid];
        let right = &line[mid..];
        'outer: for a in left.chars() {
            for b in right.chars() {
                if a == b {
                    common_letters.push(a);
                    break 'outer;
                }
            }
        }
    }
    let priorities: Vec<u32> = common_letters
        .into_iter()
        .map(|c| char_to_value(c as u8))
        .collect();

    priorities.into_iter().sum()
}

// This maybe is more idiomatic/functional,
// but readabilty is poor for me and I wrote it.
// part_2 original seems way clearer
fn part_2_rusty(input: String) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            char_to_value(
                group
                    .iter()
                    .map(|&elf_line| HashSet::from_iter(elf_line.chars()))
                    .reduce(|acc: HashSet<char>, set| acc.intersection(&set).map(|c| *c).collect())
                    .unwrap()
                    .into_iter()
                    .nth(0)
                    .unwrap() as u8,
            )
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum()
}

fn part_2(input: String) -> u32 {
    let mut group_scores: Vec<u32> = Vec::new();
    let input: Vec<&str> = input.lines().collect();

    for group_lines in input.chunks(3) {
        let intersect = group_lines
            .into_iter()
            .map(|&elf_line| HashSet::from_iter(elf_line.chars()))
            .reduce(|acc: HashSet<char>, set| acc.intersection(&set).map(|c| *c).collect())
            .unwrap();
        group_scores.push(char_to_value(intersect.into_iter().nth(0).unwrap() as u8));
    }
    group_scores.into_iter().sum()
}

fn main() {
    let input = input_txt(InputFile::Example);
    println!("part 1 {}", part_1(input.clone()));
    println!("part 2 {}", part_2(input.clone()));
    println!("part 2 {}", part_2_rusty(input));

    let input = input_txt(InputFile::Real);
    println!("part 1 {}", part_1(input.clone())); // 7848
    println!("part 2 {}", part_2(input.clone())); // 2616
    println!("part 2 {}", part_2_rusty(input)); // 2616
}

enum InputFile {
    Example,
    Real,
}

fn input_txt(input: InputFile) -> String {
    match input {
        InputFile::Example => std::fs::read_to_string("example.txt").expect("No example.txt file"),
        InputFile::Real => std::fs::read_to_string("input.txt").expect("No input.txt file"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let part_1_example_answer = 157;

        let input = input_txt(InputFile::Example);
        let result = part_1(input);
        assert_eq!(result, part_1_example_answer);
    }

    #[test]
    fn test_example_part_2() {
        let part_2_example_answer = 70;

        let input = input_txt(InputFile::Example);
        let result = part_2(input);
        assert_eq!(result, part_2_example_answer);
    }
}
