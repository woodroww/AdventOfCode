use nom::{
    branch::alt, bytes::complete::tag, character::complete::{self, anychar}, combinator::value, multi::{many1, many_till}, sequence::{delimited, separated_pair}, IResult, Parser
};

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ShouldProcess {
    Do,
    Dont,
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn instruction (input: &str) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        mul,
    ))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    // https://github.com/rust-bakery/nom/blob/main/doc/choosing_a_combinator.md
    many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(input)
}

fn part_1(input: &str) -> String {
    let (_input, instructions) = parse(input).unwrap();
    let mut result = 0;
    for i in instructions {
        match i {
            Instruction::Mul(a, b) => {
                result += a * b;
            }
            _ => {}
        }
    }
    result.to_string()
}

fn part_2(input: &str) -> String {
    let (_input, instructions) = parse(input).unwrap();
    let result = instructions.iter().fold(
        (ShouldProcess::Do, 0),
        |(process, acc), ins| match ins {
            Instruction::Mul(a, b) => {
                if process == ShouldProcess::Do {
                    (process, acc + a * b)
                } else {
                    (process, acc)
                }
            },
            Instruction::Do => (ShouldProcess::Do, acc),
            Instruction::Dont => (ShouldProcess::Dont, acc),
        },
    );
    result.1.to_string()
}

fn main() {
    let input = input_txt(InputFile::Real);

    //let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    //println!("Part 1: {}", part_1(&input));

    //let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
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
        assert_eq!(result, "161");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "196826776");
    }

    #[test]
    fn example_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part_2(&input);
        assert_eq!(result, "48");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "106780429");
    }
}
