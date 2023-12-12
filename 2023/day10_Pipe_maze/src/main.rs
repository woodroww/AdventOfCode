
/*

╭─╮
│ │
╰─╯

│     | is a vertical pipe connecting north and south.
─     - is a horizontal pipe connecting east and west.
╰     L is a 90-degree bend connecting north and east.
╯     J is a 90-degree bend connecting north and west.
╮     7 is a 90-degree bend connecting south and west.
╭     F is a 90-degree bend connecting south and east.
.     . is ground; there is no pipe in this tile.
S     S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

*/

enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl From<PipeType> for char {
    fn from(value: PipeType) -> Self {
        match value {
            PipeType::NS => '│',
            PipeType::EW => '─',
            PipeType::NE => '╰',
            PipeType::NW => '╯',
            PipeType::SW => '╮',
            PipeType::SE => '╭',
            PipeType::Ground => '.',
            PipeType::Start => 'S',
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<PipeType>> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut columns = vec![];
        for c in line.chars() {
            columns.push(
                match c {
                    '|' => PipeType::NS,
                    '-' => PipeType::EW,
                    'L' => PipeType::NE,
                    'J' => PipeType::NW,
                    '7' => PipeType::SW,
                    'F' => PipeType::SE,
                    '.' => PipeType::Ground,
                    'S' => PipeType::Start,
                    _ => panic!("illegal char"),
                }
            );
        }
        rows.push(columns);
    }
    rows
}

fn print_pipes(pipes: Vec<Vec<PipeType>>) {
    for row in pipes {
        for pipe in row {
            print!("{}", Into::<char>::into(pipe));
        }
        println!();
    }
}

fn part_1(input: &str) -> String {
    let pipes = parse_input(input);
    print_pipes(pipes);
    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = input_txt(InputFile::Example);
    //let input = input_txt(InputFile::Real);

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
