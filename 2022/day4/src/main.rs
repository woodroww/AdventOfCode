use std::ops::Range;

fn range_in_range(a: &Range<usize>, b: &Range<usize>) -> bool {
    if a.start <= b.start && a.end >= b.end {
        // a contains b
        true
    } else if b.start <= a.start && b.end >= a.end {
        // b contains a
        true
    } else {
        false
    }
}

fn overlap(a: &Range<usize>, b: &Range<usize>) -> bool {
    for i in a.clone() {
        if b.contains(&i) {
            return true;
        }
    }
    false
}

fn parse_input(input: String) -> Vec<Range<usize>> {
    let elves: Vec<usize> = input
        .lines()
        .map(|line| line.split(','))
        .flatten()
        .map(|elf_pair| elf_pair.split('-'))
        .flatten()
        .map(|nums| nums.parse::<usize>().unwrap())
        .collect();
    //println!("{:?}", elves);
    let ranges: Vec<Range<usize>> = elves
        .chunks(2)
        .map(|start_end| {
            let mut iter = start_end.into_iter();
            Range {
                start: *iter.next().unwrap(),
                end: *iter.next().unwrap() + 1,
            }
        })
        .collect();
    //println!("{:?}", ranges);
    ranges
}

fn part_1(input: String) -> usize {
    let ranges = parse_input(input);
    let contained: Vec<_> = ranges
        .chunks(2)
        .filter(|ranges| {
            let mut iter = ranges.into_iter();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            range_in_range(a, b)
        })
        .collect();
    //println!("{:?}", contained);

    contained.len()
}

fn part_2(input: String) -> usize {
    let ranges = parse_input(input);
    let contained: Vec<_> = ranges
        .chunks(2)
        .filter(|ranges| {
            let mut iter = ranges.into_iter();
            let a = iter.next().unwrap();
            let b = iter.next().unwrap();
            overlap(a, b)
        })
        .collect();
    //println!("{:?}", contained);

    contained.len()
}

enum InputFile {
    Example,
    Real,
}

fn input_txt(input: InputFile) -> String {
    match input {
        InputFile::Example => std::fs::read_to_string("example.txt").expect("No example.txt file"),
        InputFile::Real => std::fs::read_to_string("input.txt").expect("No input.txt file"),
    }
}

fn main() {
    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let part_1_example_answer = 2;

        let input = input_txt(InputFile::Example);
        let result = part_1(input);
        assert_eq!(result, part_1_example_answer);
    }
    
    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(input);
        assert_eq!(result, 444);
    }

    #[test]
    fn test_example_part_2() {
        let part_2_example_answer = 4;

        let input = input_txt(InputFile::Example);
        let result = part_2(input);
        assert_eq!(result, part_2_example_answer);
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(input);
        assert_eq!(result, 801);
    }
}
