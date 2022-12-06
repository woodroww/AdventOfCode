use std::collections::HashSet;

fn parse_distinct(input: String, num_chars: usize) -> usize {
    //println!("input: {}", input);
    let mut set: HashSet<char> = HashSet::new();
    let input = input.chars().collect::<Vec<char>>();

    for (i, four) in input.windows(num_chars).enumerate() {
        set.clear();
        let mut duplicate = false;
        for item in four {
            if set.insert(*item) == false {
                duplicate = true;
                break;
            }
        }
        //println!("duplicate: {:?} four: {:?}", duplicate, four);
        if !duplicate {
            return i + num_chars;
        }
    }
    0
}

fn part_1(input: String) -> usize {
    parse_distinct(input, 4)
}

fn part_2(input: String) -> usize {
    parse_distinct(input, 14)
}

enum InputFile {
    Example,
    Real,
}

fn input_txt(input: InputFile) -> String {
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

fn main() {
    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_example_part_1() {
        let answer = vec![7, 5, 6, 10, 11];
        let input = input_txt(InputFile::Example);
        for (i, line) in input.lines().enumerate() {
            let result = part_1(line.to_string());
            assert_eq!(result, answer[i]);
        }
	}

    #[test]
    fn test_example_part_2() {
        let answer = vec![19, 23, 23, 29, 26];
        let input = input_txt(InputFile::Example);
        for (i, line) in input.lines().enumerate() {
            let result = part_2(line.to_string());
            assert_eq!(result, answer[i]);
        }
	}
}
