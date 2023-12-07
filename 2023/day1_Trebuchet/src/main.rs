fn part_1(input: &str) -> String {
    let mut total: u16 = 0;
    for line in input.lines() {
        match line.find(char::is_numeric) {
            Some(li) => match line.rfind(char::is_numeric) {
                Some(ri) => {
                    let left = line.chars().nth(li).unwrap();
                    let right = line.chars().nth(ri).unwrap();
                    let mut concat = String::from(left);
                    concat.push(right);
                    let num = concat.parse::<u8>().unwrap();
                    total += num as u16;
                }
                None => {}
            },
            None => {}
        }
    }
    total.to_string()
}

struct Numeral(char);

#[derive(Debug)]
struct ParseNumeralError;

impl std::str::FromStr for Numeral {
    type Err = ParseNumeralError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zero" => Ok(Numeral('0')),
            "one" => Ok(Numeral('1')),
            "two" => Ok(Numeral('2')),
            "three" => Ok(Numeral('3')),
            "four" => Ok(Numeral('4')),
            "five" => Ok(Numeral('5')),
            "six" => Ok(Numeral('6')),
            "seven" => Ok(Numeral('7')),
            "eight" => Ok(Numeral('8')),
            "nine" => Ok(Numeral('9')),
            _ => Err(ParseNumeralError),
        }
    }
}

struct NumIdx {
    num: char,
    idx: usize
}

fn part_2(input: &str) -> String {
    let numerals = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut total: u16 = 0;
    for line in input.lines() {
        println!("line: {}", line);
        let mut result = vec![];
        // six68five8pbgrvl2six
        for numeral in numerals.iter() {
            match line.find(numeral) {
                Some(idx) => {
                    let n: Numeral = numeral.parse().unwrap();
                    //println!("found {} at {}", numeral, idx);
                    result.push(NumIdx { num: n.0, idx });
                },
                None => {}
            }
            match line.rfind(numeral) {
                Some(idx) => {
                    let n: Numeral = numeral.parse().unwrap();
                    result.push(NumIdx { num: n.0, idx });
                },
                None => {}
            }
        }

        match line.find(char::is_numeric) {
            Some(li) => {
                let left = line.chars().nth(li).unwrap();
                result.push(NumIdx { num: left, idx: li });
            }
            None => {}
        }
        match line.rfind(char::is_numeric) {
            Some(ri) => {
                let right = line.chars().nth(ri).unwrap();
                result.push(NumIdx { num: right, idx: ri });
            }
            None => {}
        }

        result.sort_unstable_by(|a, b| a.idx.partial_cmp(&b.idx).unwrap());

        for r in result.iter() {
            println!("{} i:{}", r.num, r.idx);
        }

        let first = result.first().unwrap();
        let last = result.last().unwrap();
        let mut concat = String::from(first.num as char);
        concat.push(last.num as char);

        let number = concat.parse::<u8>().unwrap();
        println!("{}", number);
        total += number as u16;
    }
    total.to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    //let input_example_part_2 = std::fs::read_to_string("example_2.txt").expect("No example_2.txt file");

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
    //println!("Part 2: {}", part_2(&input_example_part_2));

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
        assert_eq!(result, "142");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "54968");
    }

    #[test]
    fn example_part_2() {
        //let input = input_txt(InputFile::Example);
        let input = std::fs::read_to_string("example_2.txt").expect("No example_2.txt file");
        let result = part_2(&input);
        assert_eq!(result, "281");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "54094");
    }
}
