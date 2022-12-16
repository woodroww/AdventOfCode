// nom parsing from https://youtu.be/QVubj-UfmEU

use nom::character::complete::u32 as nom_u32;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, *,
};
use std::cmp::Ordering;

#[derive(Debug)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

// Any type that implements Ord must also implement Eq and PartialOrd
// PartialEq is required to implement Eq

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        are_ordered(self, other)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Eq for Packet { }


impl Packet {
    fn display_string(&self) -> String {
        match self {
            Packet::List(list) => {
                let len = list.len();
                let mut result = "[".to_string();
                for (i, packet) in list.iter().enumerate() {
                    result.push_str(&packet.display_string());
                    if i < len - 1 {
                        result.push_str(",");
                    }
                }
                result.push_str("]");
                format!("{}", result)
            }
            Packet::Number(num) => num.to_string(),
        }
    }
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_string())
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}", self.left, self.right)
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    alt((
        delimited(tag("["), separated_list0(tag(","), packet), tag("]"))
            .map(|vec| Packet::List(vec)),
        nom_u32.map(|num| Packet::Number(num)),
    ))(input)
}

fn pairs(input: &str) -> IResult<&str, Vec<Pair>> {
    separated_list1(
        tag("\n\n"),
        separated_pair(packet, newline, packet).map(|(p1, p2)| Pair {
            left: p1,
            right: p2,
        }),
    )(input)
}

fn are_ordered(left: &Packet, right: &Packet) -> Ordering {
    //println!("are_ordered");
    match left {
        Packet::List(left_list) => {
            //println!("left {}", a);
            match right {
                Packet::List(right_list) => {
                    //println!("right {}", b);
                    let mut undetermined = true;
                    let mut result = Ordering::Equal;
                    let mut i = 0;
                    while undetermined {
                        if i >= left_list.len() && i >= right_list.len() {
                            return Ordering::Equal;
                        } else if i >= left_list.len() {
                            return Ordering::Less;
                        } else if i >= right_list.len() {
                            return Ordering::Greater;
                        }
                        //println!("recieved two lists\n{}\n{}", &left, &right);
                        let sub_order = are_ordered(&left_list[i], &right_list[i]);
                        result = match sub_order {
                            Ordering::Equal => Ordering::Equal,
                            Ordering::Less => {
                                undetermined = false;
                                Ordering::Less
                            }
                            Ordering::Greater => {
                                undetermined = false;
                                Ordering::Greater
                            }
                        };
                        i += 1;
                    }
                    result
                }
                Packet::Number(right_num) => {
                    //println!("right {}", b_num);
                    let num = Packet::List(vec![Packet::Number(*right_num)]);
                    are_ordered(left, &num)
                }
            }
        }
        Packet::Number(left_num) => {
            //println!("left {}", a_num);
            match right {
                Packet::List(_right_list) => {
                    //println!("right {}", b);
                    let a_list = Packet::List(vec![Packet::Number(*left_num)]);
                    are_ordered(&a_list, right)
                }
                Packet::Number(right_num) => {
                    //println!("Compare {} vs {}", left_num, right_num);
                    if left_num < right_num {
                        Ordering::Less
                    } else if left_num > right_num {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            }
        }
    }
}

fn part_1(input: &str) -> String {
    let (_, pair_list /*Vec<Pair>*/) = pairs(input).unwrap();
    let mut correct: Vec<usize> = Vec::new();
    let skip = 0;
    for (i, pair) in pair_list.iter().skip(skip).enumerate() {
        //println!("== Pair {} ==", i + 1 + skip);
        let result = are_ordered(&pair.left, &pair.right);
        if result == Ordering::Less {
            correct.push(i + 1);
        }
    }
    correct.into_iter().sum::<usize>().to_string()
}

fn part_2(input: &str) -> String {
    let (_, pair_list) = pairs(input).unwrap();
    let mut packets = pair_list.into_iter().flat_map(|pair| vec![pair.left, pair.right]).collect::<Vec<_>>();
    packets.push(Packet::List( vec![Packet::List(vec![Packet::Number(2)])]));
    packets.push(Packet::List( vec![Packet::List(vec![Packet::Number(6)])]));
    packets.sort();
    let divider_idx_1 = packets.iter().position(|p| p.display_string() == "[[2]]").unwrap();
    let divider_idx_2 = packets.iter().position(|p| p.display_string() == "[[6]]").unwrap();
    ((divider_idx_1 + 1) * (divider_idx_2 + 1)).to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);
    println!("\nPart 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_works() {
        let input = input_txt(InputFile::Example);
        let (_, pair_list) = pairs(&input).unwrap();
        let mut parsed = "".to_string();
        for pair in pair_list.iter() {
            parsed.push_str(&format!("{}\n\n", pair));
        }
        assert_eq!(input, &parsed[..parsed.len() - 1]);
    }

    #[test]
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "13");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "140");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "5938");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "29025");
    }
}
