use colored::*;

struct BracketMatch {
    start: usize,
    end: usize,
}

fn find_matches(input: &str) -> Vec<BracketMatch> {
    let mut stack: Vec<usize> = Vec::new();
    let mut result = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c == '[' {
            stack.push(i);
        } else if c == ']' {
            let top = stack.pop().expect("input should be balanced");
            result.push(BracketMatch { start: top, end: i });
        }
    }
    result
}

fn outer_brackets(input: &str) -> Option<BracketMatch> {
    let mut start = 0;
    let mut found_open = false;
    let mut end = input.len() - 1;
    let mut found_close = false;

    while let Some(c) = input.chars().nth(start) {
        if c == '[' {
            found_open = true;
            break;
        } else {
            if start < input.len() - 1 {
                start += 1;
            } else {
                break;
            }
        }
    }

    while let Some(c) = input.chars().nth(end) {
        if c == ']' {
            found_close = true;
            break;
        } else {
            if end > 0 {
                end -= 1;
            } else {
                break;
            }
        }
    }

    if found_open && found_close {
        Some(BracketMatch { start, end })
    } else {
        None
    }
}

fn colorize_idxs(input: &str, front: usize, back: usize) -> String {
    let f = input.chars().nth(front).unwrap();
    let b = input.chars().nth(back).unwrap();
    let begin_chars = if front > 0 {
        input.chars().take(front).collect::<String>()
    } else {
        "".to_string()
    };
    let begin_char = input.chars().skip(front).take(1).collect::<String>();
    let middle = input
        .chars()
        .skip(front + 1)
        .take(back - front - 1)
        .collect::<String>();
    let end_char = input.chars().skip(back).take(1).collect::<String>();
    let end_chars = if back < input.len() - 1 {
        input.chars().skip(back + 1).collect::<String>()
    } else {
        "".to_string()
    };
    format!(
        "{}{}{}{}{}",
        begin_chars,
        begin_char.red(),
        middle,
        end_char.red(),
        end_chars
    )
}

enum PacketValue {
    Integer(usize),
    List(Vec<PacketValue>),
}

impl std::fmt::Display for PacketValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            PacketValue::Integer(n) => n.to_string(),
            PacketValue::List(list) => {
                let mut list_string = "[".to_string();
                let end = list.len();
                for (i, item) in list.iter().enumerate() {
                    if i == end - 1 {
                        list_string.push_str(&format!("{}", item));
                    } else {
                        list_string.push_str(&format!("{},", item));
                    }
                }
                list_string.push_str("]");
                list_string
            }
        };
        write!(f, "{}", out)
    }
}

fn check_remaining(input: &str, start: usize, end: usize) -> (Vec<usize>, Vec<usize>) {
    let a = &input[0..start];
    let b = &input[end..];
    let c = a.to_owned() + b;
    let mut left = Vec::new();
    let mut right = Vec::new();
    //print!("remain: ");
    let mut list = &mut left;
    for x in c.split(',') {
        //print!("{} ", x);
        if let Some(integer) = x.parse::<usize>().ok() {
            list.push(integer);
        } else if x.contains(']') {
            list = &mut right;
        }
    }
    //println!();
    (left, right)
}

fn parse_packet(input: &str, level: usize) -> Option<PacketValue> {
    println!("parsing {}", input);
    let matches = find_matches(input);
    let mut stack = Vec::new();

    for m in matches.iter().rev().take(1) {
        for _ in 0..level {
            print!("    ");
        }
        println!("{}", colorize_idxs(input, m.start, m.end));
        let inner_str = &input[m.start + 1..m.end];
        //println!("inner {}", inner_str);

        if inner_str.contains('[') || inner_str.contains(']') {
            let sub = parse_packet(inner_str, level + 1);
            let mut values: Vec<PacketValue> = Vec::new();

            let (left, right) = check_remaining(input, m.start, m.end);
            for item in left {
                values.push(PacketValue::Integer(item));
            }

            if let Some(result) = sub {
                values.push(result);
                //while let Some(item) = stack.pop() {
                 //   values.push(item);
                //}
            }

            for item in right {
                values.push(PacketValue::Integer(item));
            }
            stack.push(PacketValue::List(values));


        } else {
            if inner_str.contains(',') {
                let list = inner_str
                    .split(',')
                    .map(|n| PacketValue::Integer(n.parse::<usize>().unwrap()))
                    .collect();
                stack.push(PacketValue::List(list));
            } else {
                if inner_str.len() > 0 {
                    let n = inner_str.parse::<usize>().unwrap();
                    stack.push(PacketValue::Integer(n));
                } else {
                    stack.push(PacketValue::List(vec![]));
                }
            }

            check_remaining(input, m.start, m.end);
        }
    }

    let list = stack.into_iter().rev().collect();
    Some(PacketValue::List(list))
}

/*for bracket_match in brackets.iter().rev() {
    let inner_str = &input[bracket_match.start..=bracket_match.end];
    println!("{}", colorize_idxs(input, bracket_match.start, bracket_match.end));
    stack.push(inner_str);
}*/

/*let mut prev = "".to_string();
while let Some(item) = stack.pop() {
    let mut brackets = find_matches(item);
    //println!("-{}", item);
    if let Some(jam) = brackets.pop() {
        println!("pop {}[{}]{}", &item[0..jam.start], prev, &item[jam.end+1..]);
    } else {
        println!("{}", item);
    }
    prev = item.to_string();
}*/

fn in_order(left: &str, right: &str) -> bool {
    let result = parse_packet(left, 0);
    if let Some(r) = result {
        println!("result: {}", r);
    }
    println!();

    false
}

fn part_1(input: &str) -> String {
    let mut ordered_idxs: Vec<usize> = Vec::new();
    for (i, pair) in input.split("\n\n").enumerate().skip(0) {
        let (left, right) = pair.split_once("\n").unwrap();
        if in_order(left, right) {
            ordered_idxs.push(i);
        }
    }

    ordered_idxs.iter().sum::<usize>().to_string()
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
    println!("\nPart 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));

    //let input = input_txt(InputFile::Real);
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
