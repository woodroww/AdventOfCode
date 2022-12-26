fn parse_input(input: &str) -> Vec<(usize, i64)> {
    let mut nums = Vec::new();
    for (i, line) in input.lines().enumerate() {
        nums.push((i, line.parse::<i64>().unwrap()));
    }
    nums
}

fn print_vec(state: &Vec<(usize, i64)>, correct: &Vec<i64>) {
    print!("         ");
    for item in state.iter() {
        print!("{} ", item.1);
    }
    println!();

    print!("correct: ");
    for item in correct.iter() {
        print!("{} ", item);
    }
    println!();
    println!();
}

fn part_1(input: &str) -> String {
    let numbers = parse_input(input);
    let mut state = numbers.clone();
    let correct: Vec<Vec<i64>> = vec![
        vec![1, 2, -3, 3, -2, 0, 4],
        vec![2, 1, -3, 3, -2, 0, 4],
        vec![1, -3, 2, 3, -2, 0, 4],
        vec![1, 2, 3, -2, -3, 0, 4],
        vec![1, 2, -2, -3, 0, 3, 4],
        vec![1, 2, -3, 0, 3, 4, -2],
        vec![1, 2, -3, 0, 3, 4, -2],
        vec![1, 2, -3, 4, 0, 3, -2],
    ];
    print_vec(&state, &correct[0]);

    for (id, _num) in numbers.iter() {
        let index = state.iter().position(|state_value| state_value.0 == *id).unwrap();
        println!("moving {} at index {}", state[index].1, index);
        let current = state.remove(index);
        let added = index as i64 + current.1;
        //println!("added {}", added);
        let new_index = added.rem_euclid(state.len() as i64);
        println!("new_index = {}", new_index);

        state.insert(new_index as usize, current);
        print_vec(&state, &correct[*id + 1]);
    }

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
