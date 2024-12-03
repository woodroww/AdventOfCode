use intcode::{Opcode, Result, Value};

fn parse(input: &str) -> Vec<Value> {
    input
        .split(',')
        .map(|s| s.parse::<Value>().unwrap())
        .collect()
}

fn parse_opcode(input: Value) -> Result<Opcode> {
    Opcode::try_from(input % 100)
}

fn part_1(input: &str) -> String {
    let s = parse(&input[0..input.len()-1]);
    println!("{:?}", s);

    let op = parse_opcode(s[0]).unwrap();

    format!("op: {}", op)
}

fn run_intcode(mut intcode: Vec<usize>) -> Vec<usize> {
    let mut i = 0;
    loop {
        //println!("{:?}", intcode);
        // 1 addition, 2 multiply, 99 finished
        if intcode[i] == 1 || intcode[i] == 2 {
            let a = intcode[intcode[i + 1]];
            let b = intcode[intcode[i + 2]];
            let output_position = intcode[i + 3];
            if intcode[i] == 1 {
                //println!("{a} + {b} = {}", a + b);
                intcode[output_position] = a + b;
            } else {
                //println!("{a} * {b} = {}", a * b);
                intcode[output_position] = a * b;
            }
            i += 4;
        } else if intcode[i] == 99 {
            break;
        } else {
            panic!("unknown opcode {}", intcode[i]);
        }
    }
    intcode
}

fn part_2(_input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = input_txt(InputFile::Example);
    //let input = input_txt(InputFile::Real);

    println!("Part 1:\n{}", part_1(&input));
    //println!("Part 2:\n{}", part_2(&input));
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
        assert_eq!(result, "0");
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
