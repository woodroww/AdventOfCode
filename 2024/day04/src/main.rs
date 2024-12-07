use std::collections::HashMap;
use colored::*;

fn parse_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut lines = vec![];
    for line in input.lines() {
        //println!("{}", line);
        let mut char_line = vec![];
        for c in line.chars() {
            char_line.push(c);
        }
        lines.push(char_line);
    }
    lines
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }

}

const DIRECTIONS: [[Vec2; 3]; 8] = [
// XMAS
    [Vec2 { x: 1, y: 0 }, Vec2 { x: 2, y: 0 }, Vec2 { x: 3, y: 0 }],
// SAMX
    [Vec2 { x: -1, y: 0 }, Vec2 { x: -2, y: 0 }, Vec2 { x: -3, y: 0 }],
// X...
// M...
// A...
// S...
    [Vec2 { x: 0, y: 1 }, Vec2 { x: 0, y: 2 }, Vec2 { x: 0, y: 3 }],
// S...
// A...
// M...
// X...
    [Vec2 { x: 0, y: -1 }, Vec2 { x: 0, y: -2 }, Vec2 { x: 0, y: -3 }],
// X...
// .M..
// ..A.
// ...S
    [Vec2 { x: 1, y: 1 }, Vec2 { x: 2, y: 2 }, Vec2 { x: 3, y: 3 }],
// S...
// .A..
// ..M.
// ...X
    [Vec2 { x: 1, y: -1 }, Vec2 { x: 2, y: -2 }, Vec2 { x: 3, y: -3 }],
// ...X
// ..M.
// .A..
// S...
    [Vec2 { x: -1, y: -1 }, Vec2 { x: -2, y: -2 }, Vec2 { x: -3, y: -3 }],
// ...S
// ..A.
// .M..
// X...
    [Vec2 { x: -1, y: 1 }, Vec2 { x: -2, y: 2 }, Vec2 { x: -3, y: 3 }],
];

fn part_1(input: &str) -> String {
    let mut board = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    let max_y = lines.len();
    let a_line: Vec<char> = lines.iter().nth(0).unwrap().chars().collect();
    let max_x = a_line.len();

    for (y, line) in lines.iter().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            board.insert(Vec2 { x: x as isize, y: y as isize }, c);
        }
    }

    let mas = ['M', 'A', 'S'];
    let mut count = 0;
    for (position, _) in board.iter().filter(|(_, value)| **value == 'X') {
        for dir in DIRECTIONS.iter() {
            //println!();
            //print_board(&board, max_x, max_y, position, dir);
            let mut matching = 0;
            for (offset, mas_c) in dir.iter().zip(mas)  {
                let next_pos = offset.clone() + position.clone();
                if let Some(board_c) = board.get(&next_pos) {
                    //println!("mas_c: {} - [{}, {}] - {}", mas_c, next_pos.x, next_pos.y, board_c);
                    if *board_c == mas_c {
                        matching += 1;
                    }
                }
            }
            if matching == 3 {
                count += 1;
                //println!("count: {}", count);
            }
        }
    }

    count.to_string()
}

fn print_board(board: &HashMap<Vec2, char>, max_x: usize, max_y: usize, current_x: &Vec2, direction: &[Vec2; 3]) {
    for y in (0..max_y).rev() {
        for x in 0..max_x {
            let pos = Vec2 { x: x as isize, y: y as isize };
            if let Some(c) = board.get(&pos) {
                let search_dirs: Vec<Vec2> = direction.iter().map(|offset| offset.clone() + current_x.clone()).collect();
                if search_dirs.contains(&pos) {
                    print!("{}", c.to_string().yellow());
                } else if pos == *current_x {
                    print!("{}", c.to_string().red());
                } else {
                    print!("{}", c);
                }
            } else {
                panic!("what");
            }
        }
        println!();
    }
}

// M.S
// .A.
// M.S

// S.M
// .A.
// S.M

// S.S
// .A.
// M.M

// M.M
// .A.
// S.S

const DIRECTIONS_2: [Vec2; 4] = [
    Vec2 { x: -1, y: 1 },
    Vec2 { x: 1, y: -1 },

    Vec2 { x: 1, y: 1 },
    Vec2 { x: -1, y: -1 },
];

fn part_2(input: &str) -> String {
    let mut board = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    for (y, line) in lines.iter().rev().enumerate() {
        for (x, c) in line.chars().enumerate() {
            board.insert(Vec2 { x: x as isize, y: y as isize }, c);
        }
    }
    let mas = ['M', 'S'];
    let sam = ['S', 'M'];
    let mut count = 0;
    for (position, _) in board.iter().filter(|(_, value)| **value == 'A') {
        let mut matching = 0;
        for chunk in DIRECTIONS_2.chunks(2) {
            let mut sub_match = 0;
            for (offset, c) in chunk.iter().zip(mas) {
                let next_pos = offset.clone() + position.clone();
                if let Some(board_c) = board.get(&next_pos) {
                    //println!("mas_c: {} - [{}, {}] - {}", mas_c, next_pos.x, next_pos.y, board_c);
                    if *board_c == c {
                        sub_match += 1;
                    }
                }
            }
            if sub_match == 2 {
                matching += 1;
            }
            sub_match = 0;
            for (offset, c) in chunk.iter().zip(sam) {
                let next_pos = offset.clone() + position.clone();
                if let Some(board_c) = board.get(&next_pos) {
                    //println!("mas_c: {} - [{}, {}] - {}", mas_c, next_pos.x, next_pos.y, board_c);
                    if *board_c == c {
                        sub_match += 1;
                    }
                }
            }
            if sub_match == 2 {
                matching += 1;
            }
        }
        if matching == 2 {
            count += 1;
        }
    }
    count.to_string()
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
        assert_eq!(result, "18");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "2530");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "9");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "1921");
    }
}
