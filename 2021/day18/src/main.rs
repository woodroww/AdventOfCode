use colored::*;

fn add(a: String, b: String) -> String {
    format!("[{},{}]", a, b)
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);
    //println!("Part 1: {}", part_1(input.clone()));
    //let line = input.lines().nth(2).unwrap();

    for line in input.lines() {
        let result = process_line(line);
        //println!("line   {}", line);
        //println!("result {}", result);
        assert_eq!(line, format!("{}", result));
    }

    let explode_test = std::fs::read_to_string("explode.txt")
        .expect("No explode.txt file");
    let examples = explode_test.split("\n\n").map(|pair| pair.split_once("\n").unwrap());
    for (before, after) in examples {
        let result = process_line(before);
        let result = explode(&result);
    }
}

fn explode(input: &SnailFishNumber) -> SnailFishNumber {

    let stack: Vec<usize> = Vec::new();
    let number = Some(input); 

    while number.is_some() {
        let number = number.unwrap();
        
        if let Some(left) = *number.left {
            left.
        }
    }

    SnailFishNumber::default()
}

fn colorize_idxs(input: &String, front: usize, back: usize) -> String {
    let f = input.chars().nth(front).unwrap();
    let b = input.chars().nth(back).unwrap();
    let begin_chars = if front > 0 {
        input.chars().take(front).collect::<String>()
    } else {
        "".to_string()
    };
    let begin_char = input.chars().skip(front).take(1).collect::<String>();
    let middle = input.chars().skip(front+1).take(back-front-1).collect::<String>();
    let end_char = input.chars().skip(back).take(1).collect::<String>();
    let end_chars = if back < input.len() - 1 {
        input.chars().skip(back + 1).collect::<String>()
    } else {
        "".to_string()
    };
    format!("{}{}{}{}{}", begin_chars, begin_char.red(), middle, end_char.red(), end_chars)
}

#[test]
fn test_explode_1() {
    let input = "[[[[[9,8],1],2],3],4]";
    let expected = "[[[[0,9],2],3],4]";
    let result = format!("{}", explode(&process_line(input)));
    assert_eq!(expected, result);
}

#[test]
fn test_explode_2() {
    let input = "[7,[6,[5,[4,[3,2]]]]]";
    let expected = "[7,[6,[5,[7,0]]]]";
    let result = format!("{}", explode(&process_line(input)));
    assert_eq!(expected, result);
}

#[test]
fn test_explode_3() {
    let input = "[[6,[5,[4,[3,2]]]],1]";
    let expected = "[[6,[5,[7,0]]],3]";
    let result = format!("{}", explode(&process_line(input)));
    assert_eq!(expected, result);
}

#[test]
fn test_explode_4() {
    let input = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
    let expected = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
    let result = format!("{}", explode(&process_line(input)));
    assert_eq!(expected, result);
}

#[test]
fn test_explode_5() {
    let input = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
    let expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
    let result = format!("{}", explode(&process_line(input)));
    assert_eq!(expected, result);
}

fn part_1(_input: String) -> i32 {
    let simple_1 = "[1,1]\n[2,2]\n[3,3]\n[4,4]";
    let simple_1_sum = "[[[[1,1],[2,2]],[3,3]],[4,4]]"; 

    let simple_2 = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]";
    let simple_2_sum = "[[[[3,0],[5,3]],[4,4]],[5,5]]"; 

    let simple_3 = "[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]";
    let simple_3_sum = "[[[[5,0],[7,4]],[5,5]],[6,6]]";

    let input = simple_2;
    println!("{}", input);
    let (first, _) = input.split_once("\n").unwrap();
    let mut prev_line = first.to_string();
    for line in input.lines().skip(1) {
        let mut result = add(prev_line.to_string(), line.to_string());
        println!("unchecked: {}", result);
        result = check(result);
        println!("result:    {}", result);
        prev_line = result;
    }



    0
}

fn part_2(_input: String) -> i32 {
    0
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

// every snailfish number is a pair - an ordered list of two elements.
// Each element of the pair can be either a regular number or another pair.

#[derive(Debug)]
enum SnailFishType {
    Real(usize),
    SnailFish(SnailFishNumber),
    Text(String),
}

impl std::fmt::Display for SnailFishType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailFishType::Real(num) => {
                write!(f, "{}", num)
            },
            SnailFishType::SnailFish(num) => {
                write!(f, "{}", num)
            },
            SnailFishType::Text(text) => {
                write!(f, "{}", text)
            }
        }
    }
}

#[derive(Default, Debug)]
struct SnailFishNumber {
    left: Box<Option<SnailFishType>>,
    right: Box<Option<SnailFishType>>,
}

fn post_order(node: Option<&SnailFishNumber>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if node.is_none() {
        return Ok(());
    }
    let node = node.unwrap();

    write!(f, "[")?;
    if let Some(left) = &*node.left {
        match left {
            SnailFishType::Real(num) => {
                write!(f, "{}", num)?;
            },
            SnailFishType::SnailFish(l) => {
                post_order(Some(&l), f)?;
            },
            SnailFishType::Text(text) => {
                write!(f, "{}", text)?;
            }
        }
    }
    write!(f, ",")?;
    if let Some(right) = &*node.right {
        match right {
            SnailFishType::Real(num) => {
                write!(f, "{}", num)?;
            },
            SnailFishType::SnailFish(r) => {
                post_order(Some(&r), f)?;
            },
            SnailFishType::Text(text) => {
                write!(f, "{}", text)?;
            }
        }
    }
    write!(f, "]")
}

impl std::fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        post_order(Some(self), f)
    }
}

fn process_line(line: &str) -> SnailFishNumber {
    let matches = find_matches(line);
    /*matches.iter().rev().for_each(|item| {
        print!("{}", colorize_idxs(&line.to_string(), item.start, item.end));
        println!(" ({},{})", item.start, item.end);
    });
    println!();*/

    let mut stack: Vec<SnailFishNumber> = Vec::new();
    for m in matches {
        let inside = inside_match(line, &m);
        if let Some(num) = check_final(inside) {
            stack.push(num);
        } else if let Some(num) = check_unbracked_commas(inside) {
            //println!("stack {:#?}", stack);
            if let Some(SnailFishType::Real(left)) = *num.left {
                let other = stack.pop().unwrap();
                //println!("got left {}", left);
                let mut new_fish = SnailFishNumber::default();
                new_fish.left.replace(SnailFishType::Real(left));
                new_fish.right.replace(SnailFishType::SnailFish(other));
                stack.push(new_fish);
            }
            if let Some(SnailFishType::Real(right)) = *num.right {
                let other = stack.pop().unwrap();
                //println!("got right {}", right);
                let mut new_fish = SnailFishNumber::default();
                new_fish.left.replace(SnailFishType::SnailFish(other));
                new_fish.right.replace(SnailFishType::Real(right));
                stack.push(new_fish);
            }
        } else {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            let mut new_fish = SnailFishNumber::default();
            new_fish.left.replace(SnailFishType::SnailFish(left));
            new_fish.right.replace(SnailFishType::SnailFish(right));
            stack.push(new_fish);
        }
    }
    stack.pop().unwrap()
}

fn inside_match<'a>(input: &'a str, m: &BracketMatch) -> &'a str {
    &input[m.start + 1..m.end]
}

fn check_unbracked_commas(input: &str) -> Option<SnailFishNumber> {
    let find = find_comma(input);
    let rfind = rfind_comma(input);
    let mut fishy_num = SnailFishNumber::default();

    if let Some(idx) = find {
        let (left_side, _right_side) = input.split_at(idx);
        let left_side = left_side.parse::<usize>().expect("this is wrong");
        fishy_num.left.replace(SnailFishType::Real(left_side));

        let right_side = &input[idx+1..];
        fishy_num.right.replace(SnailFishType::Text(right_side.to_string()));
        return Some(fishy_num);
    }
    if let Some(idx) = rfind {
        let (_left_side, right_side) = input.split_at(idx + 1);
        let right_side = right_side.parse::<usize>().expect("this is wrong");
        fishy_num.right.replace(SnailFishType::Real(right_side));

        let left_side = &input[..idx];
        fishy_num.left.replace(SnailFishType::Text(left_side.to_string()));
        return Some(fishy_num);
    }
    None
}

fn check_final(inside: &str) -> Option<SnailFishNumber> {
    if inside.contains('[') || inside.contains(']') {
        return None;
    }
    let comma_idx = inside.find(',').expect("this shold have a comma");
    let (left_side, right_side) = inside.split_at(comma_idx);
    let right_side = &right_side[comma_idx..];
    //println!("left {}", left_side);
    //println!("right {}", right_side);
    let left_side = left_side.parse::<usize>().expect("has to be a number");
    let right_side = right_side.parse::<usize>().expect("has to be a number");
    let mut fishy_num = SnailFishNumber::default();
    fishy_num.left.replace(SnailFishType::Real(left_side));
    fishy_num.right.replace(SnailFishType::Real(right_side));
    Some(fishy_num)
}

#[test]
fn test_final_split() {
    let a = "7,8";
    let fish = check_final(a).unwrap();
    if let SnailFishType::Real(left) = fish.left.unwrap() {
        assert_eq!(left, 7);
    } else {
        assert!(false);
    }
    if let SnailFishType::Real(right) = fish.right.unwrap() {
        assert_eq!(right, 8);
    } else {
        assert!(false);
    }
}


fn rfind_comma(input: &str) -> Option<usize> {
    for (i, c) in input.chars().rev().enumerate() {
        if c == ']' {
            break;
        } else if c == ',' {
            return Some(input.len() - 1 - i);
        }
    }
    None
}

fn find_comma(input: &str) -> Option<usize> {
    for (i, c) in input.chars().enumerate() {
        if c == '[' {
            break;
        } else if c == ',' {
            return Some(i);
        }
    }
    None
}


#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_example_part_1() {
        let part_1_example_answer = 1;

        let input = input_txt(InputFile::Example);
		let result = part_1(input);
        assert_eq!(result, part_1_example_answer);
	}

    #[test]
    fn test_example_part_2() {
        let part_2_example_answer = 1;

        let input = input_txt(InputFile::Example);
		let result = part_2(input);
        assert_eq!(result, part_2_example_answer);
	}
}
