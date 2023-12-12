fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut result = vec![];
    for line in input.lines() {
        let history: Vec<isize> = line
            .split_whitespace()
            .map(|a| a.parse::<isize>().unwrap())
            .collect();
        result.push(history);
    }
    result
}

fn find_diffs(line: &Vec<isize>) -> Vec<isize> {
    let mut diffs = vec![];
    for c in line.windows(2) {
        diffs.push(c[1] - c[0]);
    }
    diffs
}

fn all_zeros(line: &Vec<isize>) -> bool {
    for n in line {
        if *n != 0 {
            return false;
        }
    }
    true
}

fn part_1(input: &str) -> String {
    let histories = parse_input(input);
    let mut new_histories = vec![];
    for line in &histories {
        let mut next_lines = vec![];
        let mut diffs = find_diffs(line);
        next_lines.push(line.clone());
        next_lines.push(diffs.clone());
        while !all_zeros(&diffs) {
            diffs = find_diffs(&diffs);
            next_lines.push(diffs.clone());
        }
        /*
        for next in &next_lines {
        println!("  {:?}", next);
        }
        */

        let mut last = None;
        for next in next_lines.iter_mut().rev() {
            if last.is_none() {
                last = next.last();
            }
            let new_value = *next.last().unwrap() + last.unwrap();
            next.push(new_value);
            last = next.last();
        }
        new_histories.push(next_lines);

        /*
        for next in &next_lines {
        println!("+ {:?}", next);
        }
        println!();
        */
    }
    let mut sum = 0;
    for history in new_histories {
        sum += history[0].last().unwrap();
    }
    sum.to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

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
        assert_eq!(result, "114");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "1921197370");
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
