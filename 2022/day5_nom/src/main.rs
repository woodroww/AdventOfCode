// https://www.youtube.com/watch?v=6b2ymQWldoE&t=487s

use nom::{
    character::complete::{alpha1, char, newline, multispace1, digit1},
    sequence::preceded,
    bytes::complete::tag,
    multi::{separated_list1, many1},
    IResult,
};

fn parse_crate(input: &str) -> IResult<&str, Option<&str>> {
    let (input, c) = nom::branch::alt((
        tag("   "),
        nom::sequence::delimited(char('['), alpha1, char(']')),
    ))(input)?;

    let result = match c {
        "   " => None,
        value => Some(value),
    };
    Ok((input, result))
}

fn line(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    let (input, result) = separated_list1(tag(" "), parse_crate)(input)?;
    Ok((input, result))
}

fn crates(input: &str) -> IResult<&str, (Vec<Vec<&str>>, Vec<Move>)> {
    let (input, crates_horizontal) = separated_list1(newline, line)(input)?;
    let (input, _) = newline(input)?;
    let (input, _numbers) = many1(preceded(multispace1, digit1))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, moves) = separated_list1(newline, move_crate)(input)?;

    let mut crates_vertical = vec![vec![]; crates_horizontal.len()];

    for horiz in crates_horizontal.into_iter().rev() {
        for (i, c) in horiz.into_iter().enumerate() {
            if let Some(crat) = c {
                crates_vertical[i].push(crat);
            }
        }
    }
    Ok((input, (crates_vertical, moves)))
}

#[derive(Debug)]
struct Move {
    number: u32,
    from: u32,
    to: u32
}

use nom::character::complete::u32 as nom_u32;

fn move_crate(input: &str) -> IResult<&str, Move> {
    let (input, _) = tag("move ")(input)?;
    let (input, number) = nom_u32(input)?;
    let (input, _) = tag(" from ")(input)?;
    let (input, from) = nom_u32(input)?;
    let (input, _) = tag(" to ")(input)?;
    let (input, to) = nom_u32(input)?;
    Ok((input, Move { number, from, to }))
}

fn part_1(input: &str) -> String {
    let (s, (crate_stacks, moves)) = crates(input).expect("everything should work");

    dbg!(crate_stacks);
    dbg!(moves);
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
    println!("Part 1: {}", part_1(&input));
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
