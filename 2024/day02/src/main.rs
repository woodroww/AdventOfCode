fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    let mut reports = vec![];
    for line in input.lines() {
        let mut levels = vec![];
        for level in line.split_whitespace() {
            levels.push(level.parse::<i32>().unwrap());
        }
        reports.push(levels);
    }
    reports
}

#[derive(Eq, PartialEq)]
enum LevelDirection {
    Increasing,
    Decreasing,
}

fn could_be_safe(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let mut dup_levels = levels.clone(); 
        dup_levels.remove(i);
        if is_safe(&dup_levels) {
            return true;
        }
    }
    false
}

fn is_safe(levels: &Vec<i32>) -> bool {
    let dir = if levels[0] - levels[1] < 0 {
        LevelDirection::Increasing
    } else {
        LevelDirection::Decreasing
    };
    for i in 1..levels.len() {
        let diff = levels[i - 1] - levels[i];
        if diff.abs() < 1 || diff.abs() > 3 {
            //println!("levels[{}] - levels[{}] = {}", i - 1, i, diff);
            return false;
        }
        let direction = if diff < 0 {
            LevelDirection::Increasing
        } else {
            LevelDirection::Decreasing
        };
        if direction != dir {
            return false;
        }
    }
    true
}

fn part_1(input: &str) -> String {
    let reports = parse_reports(input);
    let mut safe_count = 0;
    for levels in reports {
        if is_safe(&levels) {
            safe_count += 1;
        }
    }
    safe_count.to_string()
}

fn part_2(input: &str) -> String {
    let reports = parse_reports(input);
    let mut safe_count = 0;
    for levels in reports {
        if is_safe(&levels) {
            safe_count += 1;
        } else if could_be_safe(&levels) {
            safe_count += 1;
        }
    }
    safe_count.to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
        assert_eq!(result, "2");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "463");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "4");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "514");
    }
}
