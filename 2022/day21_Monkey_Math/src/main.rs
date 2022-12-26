use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, MonkeyJob> {
    let mut result = HashMap::new();
    
    for line in input.lines() {
        let (monkey, job) = line.split_once(": ").unwrap();
        let job_split = job.split(" ").collect::<Vec<&str>>();
        let job: MonkeyJob = if job_split.len() == 1 {
            MonkeyJob::Number(job_split[0].parse::<i64>().unwrap())
        } else {
            MonkeyJob::Equation(Equation {
                lhs: job_split[0].to_string(),
                rhs: job_split[2].to_string(),
                operation: job_split[1].to_string().chars().next().unwrap()
            })
        };
        result.insert(monkey.to_string(), job);
    }

    result
}

#[derive(Debug)]
struct Equation {
    lhs: String,
    rhs: String,
    operation: char,
}

#[derive(Debug)]
enum MonkeyJob {
    Number(i64),
    Equation(Equation),
}

fn do_math(name: &str, h: i64, monkeys: &HashMap<String, MonkeyJob>) -> i64 {
    if name == "humn" && h >= 0 {
        return h;
    }
    let job = monkeys.get(name).unwrap();
    let equation = match job {
        MonkeyJob::Number(n) => { return *n; }
        MonkeyJob::Equation(e) => { e },
    };

    let left = do_math(&equation.lhs, h, &monkeys);
    let right = do_math(&equation.rhs, h, &monkeys);
    let result = match equation.operation {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!("what is this?")
    };
    //println!("result: {}", result);
    result
}

fn part_1(input: &str) -> String {
    let monkeys = parse_input(input);
    let result = do_math("root", -1, &monkeys);
    result.to_string()
}

fn part_2(input: &str) -> String {
    let monkeys = parse_input(input);
    let root = monkeys.get("root").unwrap();
    let (mut left, mut right) = match root {
        MonkeyJob::Equation(e) => (e.lhs.clone(), e.rhs.clone()),
        _ => panic!("what?")
    };
    //let me = monkeys.get("humn").unwrap();

    if do_math(&right, 0, &monkeys) != do_math(&right, 1, &monkeys) {
        let tmp = left;
        left = right;
        right = tmp;
    }
    assert!(do_math(&left, 0, &monkeys) == do_math(&left, 1, &monkeys));
    assert!(do_math(&right, 0, &monkeys) == do_math(&right, 1, &monkeys));

    let target = do_math(&right, 0, &monkeys);
    let mut low = 0i64;
    let mut high = 1e+18_f64 as i64;

    while low < high {
        let mid = (low + high) / 2;
        let score = target - do_math(&left, mid, &monkeys);
        if score < 0 {
            low = mid;
        } else if score == 0 {
            println!("You've done it! {}", mid);
            break;
        } else {
            high = mid;
        }
    }


    "".to_string()
}

fn main() {
    let input = input_txt(InputFile::Example);
    //let input = input_txt(InputFile::Real);

    println!("Part 1: {}", part_1(&input));
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
        assert_eq!(result, "152");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "24947355373338");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "301");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}
}
