fn parse(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
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

fn part_1(input: &str) -> String {
    let mut intcode = parse(&input[0..input.len() - 1]);
    intcode[1] = 12;
    intcode[2] = 2;
    let result = run_intcode(intcode);
    result.first().unwrap().to_string()
}

fn part_2(input: &str) -> String {
    let intcode = parse(&input[0..input.len() - 1]);
    let result = search_combination(intcode, 19690720);
    result.to_string()
}

fn search_combination(init_memory: Vec<usize>, target: usize) -> usize {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut memory = init_memory.clone();
            memory[1] = noun;
            memory[2] = verb;

            let result = run_intcode(memory);
            if result[0] == target {
                return 100 * noun + verb;
            }
        }
    }

    0
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
        InputFile::Example => std::fs::read_to_string("example.txt").expect("No example.txt file"),
        InputFile::Real => std::fs::read_to_string("input.txt").expect("No input.txt file"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part_1() {
        let inputs = vec![
            "1,9,10,3,2,3,11,0,99,30,40,50",
            "1,0,0,0,99",
            "2,3,0,3,99",
            "2,4,4,5,99,0",
            "1,1,1,4,99,5,6,0,99",
        ];
        let expected = vec![
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![2, 0, 0, 0, 99],
            vec![2, 3, 0, 6, 99],
            vec![2, 4, 4, 5, 99, 9801],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        ];

        for (input, exp) in inputs.into_iter().zip(expected) {
            let intcode = parse(&input);
            let result = run_intcode(intcode);
            assert_eq!(result, exp);
        }
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "3790689");
    }

    /*
    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
    */

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "6533");
    }
}
