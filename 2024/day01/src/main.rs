use std::collections::HashMap;


fn part_1(input: &str) -> String {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        right.push(iter.next().unwrap().parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let result: i32 = left.iter().zip(right).map(|(l, r)| (*l-r).abs()).sum();

    result.to_string()
}

fn part_2(input: &str) -> String {
    let mut left = vec![];
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        left.push(iter.next().unwrap().parse::<i32>().unwrap());
        let entry = right.entry(iter.next().unwrap().parse::<i32>().unwrap()).or_insert(0);
        *entry += 1;
    }
    let mut scores = vec![];
    for n in left {
        let entry = right.get(&n);
        match entry {
            Some(count) => {
                scores.push(n * count);
            }
            None => {
                scores.push(0);
            }
        }
    }
    let result: i32 = scores.iter().sum();
    result.to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
        assert_eq!(result, "11");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "2375403");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "31");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "23082277");
	}
}
