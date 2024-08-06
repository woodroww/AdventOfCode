
fn part_1(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mass = line.parse::<usize>().unwrap();
        let result = (mass as f32 / 3.0).floor() as usize - 2;
        total += result;
    }
    total.to_string()
}

fn part_2(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let mass = line.parse::<isize>().unwrap();
        let result = (mass as f32 / 3.0).floor() as isize - 2;
        let additional = fuel_fuel(result);
        total += result + additional;
    }
    total.to_string()
}

fn fuel_fuel(fuel: isize) -> isize {
    let result = (fuel as f32 / 3.0).floor() as isize - 2;
    if result < 0 {
        0
    } else {
       fuel_fuel(result) + result
    }
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    //println!("Part 1:\n{}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));
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
        assert_eq!(result, "34241");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "3342946");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "51316");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "5011553");
	}
}
