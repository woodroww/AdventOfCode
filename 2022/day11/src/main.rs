use std::{collections::VecDeque, str::FromStr};

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
            _ => Self::Number(s.parse::<usize>().unwrap()),
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

impl Operation {
    fn sentence_word(&self) -> String {
        match self {
            Self::Add => "increased",
            Self::Multiply => "multiplied",
        }
        .to_string()
    }

    fn perform_operation(&self, lhs: usize, rhs: &Second) -> usize {
        match self {
            Operation::Add => match rhs {
                Second::Number(n) => lhs + n,
                Second::Old => lhs + lhs,
            },
            Operation::Multiply => match rhs {
                Second::Number(n) => lhs * n,
                Second::Old => lhs * lhs,
            },
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "*" => Self::Multiply,
            "+" => Self::Add,
            _ => unreachable!(),
        })
    }
}

struct Monkey {
    number: usize,
    items: VecDeque<usize>,
    operation: Operation,
    second: Second,
    divisible_by: usize,
    true_monkey: usize,
    false_monkey: usize,
    inspection_count: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("Monkey {}:", self.number);
        let items = format!("{:?}", self.items);
        result.push_str(&format!(
            "\n  Starting items: {}",
            &items[1..items.len() - 1]
        ));
        result.push_str(&format!(
            "\n  Operation: new = old {} {}",
            self.operation, self.second
        ));
        result.push_str(&format!("\n  Test: divisible by {}", self.divisible_by));
        result.push_str(&format!(
            "\n    If true: throw to monkey {}",
            self.true_monkey
        ));
        result.push_str(&format!(
            "\n    If false: throw to monkey {}",
            self.false_monkey
        ));
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
        let items = item_str
            .split(", ")
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<VecDeque<usize>>();

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
            inspection_count: 0,
        };
        Ok(monkey)
    }
}

#[derive(Debug)]
struct Throw {
    to_monkey: usize,
    worry: usize,
}

impl Monkey {
    fn catch(&mut self, worry: usize) {
        self.items.push_back(worry);
    }

    fn process_round_part_1(&mut self) -> Vec<Throw> {
        let mut throws = Vec::new();
        while let Some(mut worry) = self.items.pop_front() {
            self.inspection_count += 1;
            //println!("  Monkey inspects an item with a worry level of {}.", worry);
            worry = self.operation.perform_operation(worry, &self.second);
            //println!("    Worry level is {} by {} to {}", self.operation.sentence_word(), self.second, worry);
            worry = worry / 3;
            //println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry);
            if worry % self.divisible_by == 0 {
                //println!("    Current worry level is divisible by {}.", self.divisible_by);
                throws.push(Throw {
                    to_monkey: self.true_monkey,
                    worry,
                });
                //println!("    Item with worry level {} is thrown to monkey {}.", worry, self.true_monkey);
            } else {
                //println!("    Current worry level is not divisible by {}.", self.divisible_by);
                throws.push(Throw {
                    to_monkey: self.false_monkey,
                    worry,
                });
                //println!("    Item with worry level {} is thrown to monkey {}.", worry, self.false_monkey);
            }
        }
        throws
    }

    fn process_round_part_2(&mut self, product: usize) -> Vec<Throw> {
        let mut throws = Vec::new();
        while let Some(mut worry) = self.items.pop_front() {
            self.inspection_count += 1;
            worry = self.operation.perform_operation(worry, &self.second);
            worry = worry % product;
            if worry % self.divisible_by == 0 {
                throws.push(Throw {
                    to_monkey: self.true_monkey,
                    worry,
                });
            } else {
                throws.push(Throw {
                    to_monkey: self.false_monkey,
                    worry,
                });
            }
        }
        throws
    }
}

fn part_1(input: &str) -> String {
    let mut monkeys = Vec::new();
    for monkey in input.split("\n\n").collect::<Vec<&str>>() {
        let m: Monkey = monkey.parse().unwrap();
        monkeys.push(m);
    }
    /*println!("loaded monkeys:");
    for m in &monkeys {
        println!("Monkey {}: {:?}", m.number, m.items);
    }*/

    let mut throws = Vec::new();
    for round in 0..20 {
        for i in 0..monkeys.len() {
            //println!("Monkey: {}", monkeys[i].number);
            throws.extend(monkeys[i].process_round_part_1());
            for throw in throws.drain(..) {
                //println!("throw {:?}", throw);
                monkeys[throw.to_monkey].catch(throw.worry);
            }
        }
        /*println!("After round {}:", round + 1);
        for m in &monkeys {
            println!("Monkey {}: {:?}", m.number, m.items);
        }*/
    }

    monkeys.sort_by_key(|m| m.inspection_count);
    let most_inspected: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspection_count)
        .fold(1, |acc, count| acc * count);
    format!("{}", most_inspected)
}

fn part_2(input: &str) -> String {
    let mut monkeys = Vec::new();
    for monkey in input.split("\n\n").collect::<Vec<&str>>() {
        let m: Monkey = monkey.parse().unwrap();
        monkeys.push(m);
    }

    // thanks reddit
    let product = monkeys.iter().map(|m| m.divisible_by).product();

    let mut throws = Vec::new();
    for _round in 0..10_000 {
        for i in 0..monkeys.len() {
            throws.extend(monkeys[i].process_round_part_2(product));
            for throw in throws.drain(..) {
                monkeys[throw.to_monkey].catch(throw.worry);
            }
        }
    }
    monkeys.sort_by_key(|m| m.inspection_count);
    let most_inspected: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspection_count)
        .fold(1, |acc, count| acc * count);
    format!("{}", most_inspected)
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
    let input = input_txt(InputFile::Real);
    //let input = input_txt(InputFile::Real);

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

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
        assert_eq!(result, "10605");
    }

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "2713310158");
    }

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "58322");
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "13937702909");
    }
}
