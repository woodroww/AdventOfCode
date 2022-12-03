
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
    let input = input_txt(InputFile::Real);
    println!("part 1 {}", part_1(input.clone()));
    println!("part 2 {}", part_2(input));
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn test_example_part_1() {
        let part_1_example_answer = 1;

        let input = input_txt(InputFile::Example);
		let result = part_1(input);
        assert_eq!(result, part_1_example_answer);
	}

    #[test]
    fn test_example_part_2() {
        let part_2_example_answer = 1;

        let input = input_txt(InputFile::Example);
		let result = part_2(input);
        assert_eq!(result, part_2_example_answer);
	}
}
