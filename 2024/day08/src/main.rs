use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Location {
    x: usize,
    y: usize,
}

fn part_1(input: &str) -> String {
    let mut frequencies = HashSet::new();
    let mut board: HashMap<char, Vec<Location>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                _ => {
                    frequencies.insert(c);
                    board.entry(c).and_modify(|v| v.push(Location { x, y})).or_insert(vec![Location { x, y}]);
                }
            }
        }
    }
    println!("{:#?}", board);




    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = input_txt(InputFile::Example);
    //let input = input_txt(InputFile::Real);

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}

pub enum InputFile {
    Example,
    Real,
}

pub fn input_txt(input: InputFile) -> String {
    match input {
        InputFile::Example => {
            std::fs::read_to_string("example.txt")
                .expect("No example.txt file")
        },
        InputFile::Real => {
            std::fs::read_to_string("input.txt")
                .expect("No input.txt file")
        },
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
		let result = part_1(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}
}
