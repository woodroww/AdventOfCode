// https://www.youtube.com/watch?v=Xm4jrjohDN8

use nom::{bytes::complete::tag, character::complete::newline, IResult, sequence::separated_pair};
use std::ops::RangeInclusive;

fn section_assignments(
    input: &str,
) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, ranges) = nom::multi::separated_list1(newline, line)(input)?;
    Ok((input, ranges))
}

fn line_alt(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (start, end)) = separated_pair(sections, tag(","), sections)(input)?;
    Ok((input, (start, end)))
}

fn line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, start) = sections(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, end) = sections(input)?;
    Ok((input, (start, end)))
}

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = nom::character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = nom::character::complete::u32(input)?;
    Ok((input, start..=end))
}

fn part_1(input: &str) -> String {
    let (_, assignments) = section_assignments(input).unwrap();
    for a in assignments {
        println!(
            "{}..={},{}..={}",
            a.0.start(),
            a.0.end(),
            a.1.start(),
            a.1.end()
        );
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
