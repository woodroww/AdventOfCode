use itertools::Itertools;

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let mut result = vec![];
    for line in input.lines() {
        let mut iter = line.split(":");
        let test_value = iter.next().unwrap().parse::<u64>().unwrap();
        let mut numbers = vec![];
        for n_str in iter.next().unwrap().split_ascii_whitespace() {
            let n = n_str.parse::<u64>().unwrap();
            numbers.push(n);
        }
        result.push((test_value, numbers));
    }
    result
}

fn process(input: &str, operations: Vec<char>) -> String {
    let lines = parse(input);
    let mut sum = 0;
    for (test_value, numbers) in lines {
        let products = (0..numbers.len() - 1)
            .map(|_| operations.clone())
            .multi_cartesian_product()
            .collect::<Vec<Vec<char>>>();
        for s in products.into_iter() {
            let mut ops = s.iter();
            let result = numbers
                .iter()
                .copied()
                .reduce(|a, b| match ops.next().unwrap() {
                    '*' => a * b,
                    '+' => a + b,
                    'c' => {
                        let mut a_string = a.to_string();
                        a_string.push_str(&b.to_string());
                        a_string.parse::<u64>().unwrap()
                    }
                    _ => panic!(),
                });
            if test_value == result.unwrap() {
                sum += test_value;
                break;
            }
        }
    }
    sum.to_string()
}

fn part_1(input: &str) -> String {
    //let a = ["mul", "cat", "add"];
    let operations = vec!['*', '+'];
    process(input, operations)
}

fn part_2(input: &str) -> String {
    let operations = vec!['*', '+', 'c'];
    process(input, operations)
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
        InputFile::Example => std::fs::read_to_string("example.txt").expect("No example.txt file"),
        InputFile::Real => std::fs::read_to_string("input.txt").expect("No input.txt file"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "3749");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "66343330034722");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "11387");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "637696070419031");
    }
}
