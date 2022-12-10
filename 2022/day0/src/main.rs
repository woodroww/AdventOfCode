
fn part_1(input: String) -> i32 {
    0
}

fn part_2(input: String) -> i32 {
    0
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
    println!("Part 1: {}", part_1(input.clone()));
    //println!("Part 2: {}", part_2(input));

    //let input = input_txt(InputFile::Real);
    //println!("Part 1: {}", part_1(input.clone()));
    //println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_example_part_1() {
        let answer = 1;

        let input = input_txt(InputFile::Example);
		let result = part_1(input);
        assert_eq!(result, answer);
	}

    #[test]
    fn test_example_part_2() {
        let answer = 1;

        let input = input_txt(InputFile::Example);
		let result = part_2(input);
        assert_eq!(result, answer);
	}

    #[test]
    fn test_real_part_1() {
        let answer = 1;

        let input = input_txt(InputFile::Real);
		let result = part_1(input);
        assert_eq!(result, answer);
	}

    #[test]
    fn test_real_part_2() {
        let answer = 1;

        let input = input_txt(InputFile::Real);
		let result = part_2(input);
        assert_eq!(result, answer);
	}
}
