fn parse_instructions(input: &str) -> Vec<isize> {
    let mut instructions: Vec<isize> = Vec::new();
    for line in input.lines() {
        if line.starts_with("a") {
            let (_, v) = line.rsplit_once(' ').unwrap();
            instructions.push(0);
            instructions.push(v.parse::<isize>().unwrap());
        } else /* noop */ {
            instructions.push(0);
        }
    }
    instructions
}

fn part_1(input: &str) -> String {
    let mut register = 1;
    let mut cycle = 1;
    let instructions = parse_instructions(input);
    let inspect_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut strength_sum = 0;
    for (_i, instruction) in instructions.iter().enumerate() {
        cycle += 1;
        register += instruction;
        if inspect_cycles.contains(&cycle) {
            strength_sum += cycle * register;
        }
    }
    format!("{}", strength_sum)
}

fn part_2(input: &str) -> String {
    let mut register = 1;
    let mut x_pixel = 0;
    let mut result = "".to_string();
    let instructions = parse_instructions(input);
    for instruction in instructions.iter() {
        if (register - 1..=register + 1).contains(&x_pixel) {
            result.push('#');
        } else {
            result.push('.');
        }
        register += instruction;
        x_pixel += 1;
        if x_pixel == 40 {
            x_pixel = 0;
            result.push('\n');
        }
    }
    result
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

fn main() {
    let input = input_txt(InputFile::Example);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));

    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_example_part_1() {
        let input = input_txt(InputFile::Example);
		let result = part_1(&input);
        assert_eq!(result, "13140");
	}

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#);
	}

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "14540");
	}

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, r#"####.#..#.####.####.####.#..#..##..####.
#....#..#....#.#.......#.#..#.#..#....#.
###..####...#..###....#..####.#......#..
#....#..#..#...#.....#...#..#.#.....#...
#....#..#.#....#....#....#..#.#..#.#....
####.#..#.####.#....####.#..#..##..####.
"#);
	}
}
