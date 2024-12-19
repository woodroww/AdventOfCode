use std::collections::HashSet;


#[derive(PartialEq, Eq)]
enum Square {
    Empty,
    Guard,
    Obstruction,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Guard {
    loc: Location,
    dir: GuardDirection,
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Empty => write!(f, "."),
            Square::Guard => write!(f, "^"),
            Square::Obstruction => write!(f, "#"),
        }
    }
}



fn parse(input: &str) -> (Vec<Vec<Square>>, Guard) {
    let mut result = vec![];
    let mut guard = None;
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let square = match c {
                '.' => Square::Empty,
                '#' => Square::Obstruction,
                '^' => Square::Guard,
                _ => panic!("invalid input"),
            };
            if square == Square::Guard {
                guard = Some(Guard { loc: Location {x, y}, dir: GuardDirection::Up } );
            }
            row.push(square);
        }
        result.push(row);
    }
    (result, guard.unwrap())
}

fn print_board(board: &Vec<Vec<Square>>) {
    for line in board.iter() {
        for space in line.iter() {
            print!("{}", space);
        }
        println!();
    }
}

fn part_1(input: &str) -> String {
    let (board, mut guard) = parse(input);
    let board_height = board.len();
    let board_width = board.iter().nth(0).unwrap().len();
    let mut visited = HashSet::new();
    visited.insert(guard.loc);

    let mut guard_on_board = true;
    while guard_on_board {
        let mut new_guard = guard;
        match guard.dir {
            GuardDirection::Up => {
                if new_guard.loc.y > 0 {
                    new_guard.loc.y -= 1;
                } else {
                    guard_on_board = false;
                }
            }
            GuardDirection::Down => {
                if new_guard.loc.y < board_height - 1 {
                    new_guard.loc.y += 1;
                } else {
                    guard_on_board = false;
                }
            }
            GuardDirection::Left => {
                if new_guard.loc.x > 0 {
                    new_guard.loc.x -= 1;
                } else {
                    guard_on_board = false;
                }
            }
            GuardDirection::Right => {
                if new_guard.loc.x < board_width - 1 {
                    new_guard.loc.x += 1;
                } else {
                    guard_on_board = false;
                }
            }
        }
        if guard_on_board {
            let guard_square = board.iter().nth(new_guard.loc.y).unwrap().iter().nth(new_guard.loc.x).unwrap();
            if *guard_square == Square::Obstruction {
                guard.dir = match guard.dir {
                    GuardDirection::Up => GuardDirection::Right,
                    GuardDirection::Down => GuardDirection::Left,
                    GuardDirection::Left => GuardDirection::Up,
                    GuardDirection::Right => GuardDirection::Down,
                };
            } else {
                guard = new_guard;
            }
            visited.insert(guard.loc);
        }
    }

    visited.len().to_string()
}

fn part_2(input: &str) -> String {
    let (board, guard_start) = parse(input);
    let board_height = board.len();
    let board_width = board.iter().nth(0).unwrap().len();
    let mut guard_loop_count = 0;

    for obstruction_y in 0..board_height {
        for obstruction_x in 0..board_width {
            let mut visited = HashSet::new();
            let mut guard = guard_start;
            visited.insert(guard.loc);
            let mut guard_visited = HashSet::new();
            let mut guard_on_board = true;
            let mut guard_in_loop = false;
            while guard_on_board {
                if guard_visited.contains(&guard) {
                    guard_in_loop = true;
                    break;
                } else {
                    guard_visited.insert(guard);
                }
                let mut new_guard = guard;
                match guard.dir {
                    GuardDirection::Up => {
                        if new_guard.loc.y > 0 {
                            new_guard.loc.y -= 1;
                        } else {
                            guard_on_board = false;
                        }
                    }
                    GuardDirection::Down => {
                        if new_guard.loc.y < board_height - 1 {
                            new_guard.loc.y += 1;
                        } else {
                            guard_on_board = false;
                        }
                    }
                    GuardDirection::Left => {
                        if new_guard.loc.x > 0 {
                            new_guard.loc.x -= 1;
                        } else {
                            guard_on_board = false;
                        }
                    }
                    GuardDirection::Right => {
                        if new_guard.loc.x < board_width - 1 {
                            new_guard.loc.x += 1;
                        } else {
                            guard_on_board = false;
                        }
                    }
                }
                if guard_on_board {
                    let guard_square = board.iter().nth(new_guard.loc.y).unwrap().iter().nth(new_guard.loc.x).unwrap();
                    let loc = Location { x: obstruction_x, y: obstruction_y };
                    if *guard_square == Square::Obstruction || new_guard.loc == loc {
                        guard.dir = match guard.dir {
                            GuardDirection::Up => GuardDirection::Right,
                            GuardDirection::Down => GuardDirection::Left,
                            GuardDirection::Left => GuardDirection::Up,
                            GuardDirection::Right => GuardDirection::Down,
                        };
                    } else {
                        guard = new_guard;
                    }
                    visited.insert(guard.loc);
                }
            }
            if guard_in_loop {
                guard_loop_count += 1;
                //println!("guard in loop");
            }
        }
    }

    guard_loop_count.to_string()
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
        assert_eq!(result, "41");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "5162");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "6");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "1909");
	}
}
