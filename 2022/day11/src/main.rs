use std::str::FromStr;

enum Second {
    Number(usize),
    Old,
}

impl std::fmt::Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Number(n) => format!("{}", n),
            Self::Old => "old".to_string(),
        };
        write!(f, "{}", c)
    }
}

impl FromStr for Second {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "old" => Self::Old,
            _ => {
                Self::Number(s.parse::<usize>().unwrap())
            }
        })
    }
}

enum Operation {
    Add,
    Multiply,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Add => '+',
            Self::Multiply => '*',
        };
        write!(f, "{}", c)
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "*" => Self::Multiply,
            "+" => Self::Add,
            _ => unreachable!()
        })
    }
}

struct Monkey {
    number: usize,
    items: Vec<usize>,
    operation: Operation,
    second: Second,
    divisible_by: usize,
    true_monkey: usize,
    false_monkey: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("Monkey {}:", self.number);
        let items = format!("{:?}", self.items);
        result.push_str(&format!("\n  Starting items: {}", &items[1..items.len() - 1]));
        result.push_str(&format!("\n  Operation: new = old {} {}", self.operation, self.second));
        result.push_str(&format!("\n  Test: divisible by {}", self.divisible_by));
        result.push_str(&format!("\n    If true: throw to monkey {}", self.true_monkey));
        result.push_str(&format!("\n    If false: throw to monkey {}", self.false_monkey));
        write!(f, "{}", result)
    }
}
impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        
        // Monkey #:
        let (_, number_str) = lines[0].split_once(" ").unwrap();
        let number = number_str[..number_str.len() - 1].parse::<usize>().unwrap();

        // Starting items:
        let item_str = lines[1].split_once(": ").unwrap().1;
        let items = item_str.split(", ").map(|i| i.parse::<usize>().unwrap()).collect();

        // Operation:
        let op_line: Vec<&str> = lines[2].split_whitespace().collect();
        let operation: Operation = op_line[4].parse().unwrap();
        let second: Second = op_line[5].parse().unwrap();

        // Test:
        let divisible_by: usize = lines[3].rsplit_once(" ").unwrap().1.parse().unwrap();

        // next monkey
        let true_monkey: usize = lines[4].rsplit_once(" ").unwrap().1.parse().unwrap();
        let false_monkey: usize = lines[5].rsplit_once(" ").unwrap().1.parse().unwrap();

        let monkey = Monkey {
            number,
            operation,
            second,
            items,
            divisible_by,
            true_monkey,
            false_monkey,
        };
        Ok(monkey)
    }
}

fn part_1(input: &str) -> String {
    let mut monkeys = Vec::new();
    for monkey in input.split("\n\n").collect::<Vec<&str>>() {
        let m: Monkey = monkey.parse().unwrap();
        monkeys.push(m);
    }


    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
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
    //let input = input_txt(InputFile::Real);

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));

    //println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
