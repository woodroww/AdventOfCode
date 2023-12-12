use std::collections::HashSet;

pub fn neighbors(x: usize, y: usize, x_bound: usize, y_bound: usize) -> Vec<(usize, usize)> {
    let mut dirs = Vec::new();
    if let Some(x) = x.checked_sub(1) {
        dirs.push((x, y));
    }
    if let Some(y) = y.checked_sub(1) {
        dirs.push((x, y));
    }
    if let Some(x) = x.checked_add(1) {
        if x < x_bound {
            dirs.push((x, y));
        }
    }
    if let Some(y) = y.checked_add(1) {
        if y < y_bound {
            dirs.push((x, y));
        }
    }
    if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_sub(1)) {
        dirs.push((x, y));
    }
    if let (Some(x), Some(y)) = (x.checked_add(1), y.checked_sub(1)) {
        dirs.push((x, y));
    }
    if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_add(1)) {
        dirs.push((x, y));
    }
    if let (Some(x), Some(y)) = (x.checked_add(1), y.checked_add(1)) {
        dirs.push((x, y));
    }

    dirs
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut rows = vec![];
    for line in input.lines() {
        let mut cols = vec![];
        for c in line.chars() {
            cols.push(c);
        }
        rows.push(cols);
    }
    rows
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct NumberLocation {
    number: Option<u16>,
    row: usize,
    col_start: usize,
    col_end: usize,
}

fn parse_row(row: &Vec<char>, row_index: usize) -> Vec<NumberLocation> {
    let mut number_location = None;
    let mut result = vec![];
    let mut number = String::new();
    for (i, c) in row.iter().enumerate() {
        if c.is_digit(10) {
            if number_location.is_none() {
                number = String::new();
                number_location = Some(NumberLocation {
                    number: None,
                    row: row_index,
                    col_start: i,
                    col_end: 0,
                })
            }
            number.push(*c);
        } else {
            if let Some(location) = &mut number_location {
                location.number = number.parse::<u16>().ok();
                location.col_end = i - 1;
                result.push(location.clone());
                number_location = None;
            }
        }
    }
    result
}

fn part_1(input: &str) -> String {
    let parsed = parse(input);
    let row_zero = parsed.first().unwrap();
    let x_bound = row_zero.len();
    let y_bound = parsed.len();
    let mut num_locations = vec![];

    for (i, row) in parsed.iter().enumerate() {
        let mut row_locations = parse_row(&row, i);
        num_locations.append(&mut row_locations)
    }

    let mut symbols = vec![];
    for (i, row) in parsed.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            match col {
                '*' | '#'| '+'| '$' => {
                    symbols.push((i, j));
                }
                _ => {}
            }
        }
    }
    println!("symbol locations: {:?}", symbols);
    let mut engine_parts = HashSet::new();

    for (symbol_row, symbol_col) in &symbols {
        let neighbors = neighbors(*symbol_row, *symbol_col, x_bound, y_bound);

        println!("symbols_row:{} symbol_col:{} neighbors{:?}", *symbol_row, *symbol_col, neighbors);
        for (x, y) in neighbors {
            for loc in &num_locations {
                let row_distance = (loc.row as isize - x as isize).abs();
                println!("{:?}", loc);
                println!("row: {} - y: {} = row_distance: {}", loc.row, y, row_distance);
                if row_distance <= 1 {
                    println!("({}, {})", x, y);
                    if loc.col_start <= y && loc.col_end >= y {
                        // winner
                        engine_parts.insert(loc);
                    }
                }
            }
        }
    }
    println!("engine_parts {:#?}", engine_parts);

    /*
  0123456789
0 467..114..
1 ...*......
2 ..35..633.
3 ......#...
4 617*......
5 .....+.58.
6 ..592.....
7 ......755.
8 ...$.*....
9 .664.598..

    for loc in locations {
        for (symbol_row, symbol_col) in &symbols {
            let neighbors = neighbors(*symbol_row, *symbol_col, x_bound, y_bound);
            for (x, y) in neighbors {
                let row_distance = (loc.row as isize - x as isize).abs();
                //println!("row_distance: {}", row_distance);
                if (loc.row as isize - x as isize).abs() <= 1 {
                    if loc.col_start <= y && loc.col_end >= y {
                        // winner
                        println!("jambones {:?}", loc);
                    }
                }
            }
        }
    }*/

    let mut sum = 0;
    sum.to_string()
}

/*
            match c {
                '*' | '#'| '+'| '$' => {
                    print!("{}:", c);
                    let n = neighbors(j, i, x_bound, y_bound);
                    for (x, y) in n {
                        let what = &parsed[y];
                        let jam = what[x];
                        if jam != '.' {
                            print!("{}", jam);
                            sum += jam.to_digit(10).unwrap();
                        }
                    }
                    println!();
                }
                _ => {}
            }
*/

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
