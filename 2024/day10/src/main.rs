fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part_1(input: &str) -> String {
    let heights = parse(input);
    let mut heads = vec![];
    for (y, line) in heights.iter().enumerate() {
        for (x, col) in line.iter().enumerate() {
            if *col == 0 {
                heads.push((x, y));
            }
        }
    }
    println!("{:?}", heights);
    println!("{:?}", heads);

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
