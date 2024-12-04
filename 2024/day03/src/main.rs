
fn part_1(input: &str) -> String {
    let mut numbers = vec![];
    for line in input.lines() {
        println!("{}", line);
        let pat = "mul(";
        for first in line.split(pat) {
            println!("first \t{}", first);
            if let Some(second) = first.split(')').next() {
                println!("second \t{}", second);
                let mut second_split = second.split(',');
                let n1 = second_split.next();
                let n2 = second_split.next();
                match (n1, n2) {
                    (Some(a), Some(b)) => {
                        println!("split \t{}, {}", a, b);
                        let anpoop = a.parse::<i32>();
                        let bnpoop = b.parse::<i32>();
                        match (anpoop, bnpoop) {
                            (Ok(an), Ok(bn)) => {
                                if an < 1000 && bn < 1000 {
                                    println!("\t\t{}, {}", an, bn);
                                    numbers.push(an);
                                    numbers.push(bn);
                                } else {
                                    println!("\t\tnothing");
                                }
                            }
                            _ => {
                            }
                        }
                    }
                    _ => {
                    }
                }
            }
        }
    }
    let mut result = 0;
    for chunk in numbers.chunks(2) {
        let mut iter = chunk.iter();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        result += a * b;
    }
    format!("{:?}", result).to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    // 197643112 too high
    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
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
        assert_eq!(result, "161");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}
}
