use std::str::FromStr;

#[derive(Debug)]
struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let count = split.nth(1).unwrap().parse::<usize>().unwrap();
        let from = split.nth(1).unwrap().parse::<usize>().unwrap();
        let to = split.nth(1).unwrap().parse::<usize>().unwrap();
        let m = Movement { count, from: from - 1, to: to - 1 };
        Ok(m)
    }
}

fn parse_input(input: String) -> (Vec<Movement>, Vec<Vec<char>>) {
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    //println!("{}", stacks);

    let lines: Vec<&str> = stacks.lines().rev().collect();
    //println!("{:?}", lines);

    let stack_idxs = lines
        .iter()
        .nth(0)
        .unwrap()
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if c.is_numeric() { Some(i) } else { None })
        .collect::<Vec<usize>>();
    //println!("{:?}", stack_idxs);

    let num_stacks = stack_idxs.len();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(num_stacks);
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for line in lines {
        let mut j = 0;
        for i in stack_idxs.iter() {
            let c = line.chars().nth(*i).unwrap();
            if c.is_alphabetic() {
                stacks.iter_mut().nth(j).unwrap().push(c);
            }
            j += 1;
        }
    }

    /*for s in &stacks {
        println!("stacks {:?}", s);
    }*/

    let moves = moves
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Movement>>();

    (moves, stacks)
}

fn stack_tops(stacks: &Vec<Vec<char>>) -> String {
    let mut tops = String::new();
    for s in stacks.iter() {
        //println!("stacks {:?}", s);
        tops.push(*s.last().unwrap());
    }
    tops
}

fn perform_moves_part_1(moves: Vec<Movement>, stacks: &mut Vec<Vec<char>>) {
    for m in moves {
        for _ in 0..m.count {
            let from_stack = stacks.iter_mut().nth(m.from).unwrap();
            let item = from_stack.pop().unwrap();
            let to_stack = stacks.iter_mut().nth(m.to).unwrap();
            to_stack.push(item);
        }
    }
}

fn perform_moves_part_2(moves: Vec<Movement>, stacks: &mut Vec<Vec<char>>) {
    for m in moves {
        let mut move_stack: Vec<char> = Vec::new();
        for _ in 0..m.count {
            let from_stack = stacks.iter_mut().nth(m.from).unwrap();
            let item = from_stack.pop().unwrap();
            move_stack.push(item);
        }
        let to_stack = stacks.iter_mut().nth(m.to).unwrap();
        for item in move_stack.iter().rev() {
            to_stack.push(*item);
        }
    }
}


fn part_1(input: String) -> String {
    let (moves, mut stacks) = parse_input(input);
    perform_moves_part_1(moves, &mut stacks);
    let tops = stack_tops(&stacks);
    format!("{}", tops)
}

fn part_2(input: String) -> String {
    let (moves, mut stacks) = parse_input(input);
    perform_moves_part_2(moves, &mut stacks);
    let tops = stack_tops(&stacks);
    format!("{}", tops)
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

fn main() {
    let input = input_txt(InputFile::Example);
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));

    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let part_1_example_answer = "CMZ";
        let input = input_txt(InputFile::Example);
        let result = part_1(input);
        assert_eq!(result, part_1_example_answer);
    }

    #[test]
    fn test_real_part_1() {
        let part_1_real_answer = "TLFGBZHCN";
        let input = input_txt(InputFile::Real);
        let result = part_1(input);
        assert_eq!(result, part_1_real_answer);
    }

    #[test]
    fn test_example_part_2() {
        let part_2_example_answer = "MCD";
        let input = input_txt(InputFile::Example);
        let result = part_2(input);
        assert_eq!(result, part_2_example_answer);
    }

    #[test]
    fn test_real_part_2() {
        let part_2_real_answer = "QRQFHFWCL";
        let input = input_txt(InputFile::Real);
        let result = part_2(input);
        assert_eq!(result, part_2_real_answer);
    }
}
