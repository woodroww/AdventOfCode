use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

use nom::character::complete::u32 as nom_u32;

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Down(&'a str),
    Root,
}

#[derive(Debug)]
enum Files<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) =
        separated_pair(nom_u32, tag(" "), is_a("abcdefghijklmnopqrstuvwxyz."))(input)?;
    Ok((input, Files::File { size, name }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, cmds))
}

#[derive(Debug)]
enum FileSystem<'a> {
    File { size: u32, name: &'a str },
    Dir(&'a str),
}

#[derive(Debug)]
struct File<'a> {
    size: u32,
    name: &'a str,
}

fn part_1(input: &str) -> String {
    let cmds = commands(input).unwrap().1;
    let mut context: Vec<String> = vec![];
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.push("".to_string());
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name.to_string());
            }
            Operation::Ls(files) => {
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            let mut key = "".to_string();
                            let length = context.len();
                            for (i, dir) in context.iter().enumerate() {
                                key.push_str(dir);
                                if i != length - 1 {
                                    key.push_str("/");
                                }
                            }
                            directories
                                .entry(key.to_string())
                                .and_modify(|v| v.push(File { size: *size, name }))
                                .or_insert(vec![File { size: *size, name }]);
                        }
                        Files::Dir(_) => {}
                    }
                }
            }
        }
    }

    dbg!(&directories);
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
    let input = input_txt(InputFile::Real);
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
