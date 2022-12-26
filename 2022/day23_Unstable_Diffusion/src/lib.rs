use std::collections::{HashSet, HashMap};


#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub fn max_xy_min_xy(elves: &HashSet<Position>) -> (isize, isize, isize, isize) {
    let min_x = elves
        .iter()
        .map(|s| s.x.min(s.x))
        .min()
        .unwrap();
    let max_x = elves
        .iter()
        .map(|s| s.x.max(s.x))
        .max()
        .unwrap();
    let min_y = elves
        .iter()
        .map(|s| s.y.min(s.y))
        .min()
        .unwrap();
    let max_y = elves
        .iter()
        .map(|s| s.y.max(s.y))
        .max()
        .unwrap();
    (max_x, max_y, min_x, min_y)
}

pub fn parse_input(input: &str) -> HashSet<Position> {
    let mut result = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert(Position { x: col as isize, y: row as isize });
            }
        }
    }
    result
}

pub fn print_grid(elves: &HashSet<Position>) {
    let (max_x, max_y, min_x, min_y) = max_xy_min_xy(&elves);
    //println!("max:({},{}) min:({},{})", max_x, max_y, min_x, min_y);

    let x_size = max_x - min_x + 1;
    let y_size = max_y - min_y + 1;
    let x_offset = min_x;
    let y_offset = min_y;
    //println!("x_size:{} y_size:{} x_offset:{} y_offset:{}", x_size, y_size, x_offset, y_offset);

    for row in 0..y_size {
        for col in 0..x_size {
            if elves.contains(&Position { x: col as isize + x_offset, y: row as isize + y_offset }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub struct ElfMove {
    pub from: Position,
    pub to: Position,
}

impl std::fmt::Display for ElfMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "from: {}, to: {}", self.from, self.to)
    }
}

pub fn consider_n(elf: &Position, elves: &HashSet<Position>) -> Option<ElfMove> {
    let n = Position { x: elf.x, y: elf.y - 1 };
    let ne = Position { x: elf.x + 1, y: elf.y - 1 };
    let nw = Position { x: elf.x - 1, y: elf.y - 1 };
    if !elves.contains(&n) && !elves.contains(&ne) && !elves.contains(&nw) {
        Some(ElfMove { from: elf.clone(), to: n })
    } else {
        None
    }
}

pub fn consider_s(elf: &Position, elves: &HashSet<Position>) -> Option<ElfMove> {
    let s = Position { x: elf.x, y: elf.y + 1 };
    let se = Position { x: elf.x + 1, y: elf.y + 1 };
    let sw = Position { x: elf.x - 1, y: elf.y + 1 };
    if !elves.contains(&s) && !elves.contains(&se) && !elves.contains(&sw) {
        Some(ElfMove { from: elf.clone(), to: s })
    } else {
        None
    }
}

pub fn consider_w(elf: &Position, elves: &HashSet<Position>) -> Option<ElfMove> {
    let w = Position { x: elf.x - 1, y: elf.y };
    let nw = Position { x: elf.x - 1, y: elf.y - 1 };
    let sw = Position { x: elf.x - 1, y: elf.y + 1 };
    if !elves.contains(&w) && !elves.contains(&nw) && !elves.contains(&sw) {
        Some(ElfMove { from: elf.clone(), to: w })
    } else {
        None
    }
}

fn consider_e(elf: &Position, elves: &HashSet<Position>) -> Option<ElfMove> {
    let e = Position { x: elf.x + 1, y: elf.y };
    let ne = Position { x: elf.x + 1, y: elf.y - 1 };
    let se = Position { x: elf.x + 1, y: elf.y + 1 };
    if !elves.contains(&e) && !elves.contains(&ne) && !elves.contains(&se) {
        Some(ElfMove { from: elf.clone(), to: e })
    } else {
        None
    }
}

// zero indexed rounds
//  1   N, S, W, E
//  2   S, W, E, N
//  3   W, E, N, S
//  4   E, N, S, W
pub fn propose_move(elf: &Position, elves: &HashSet<Position>, round: usize) -> Option<ElfMove> {
    if round % 4 == 0 {
        if let Some(elf_move) = consider_n(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_s(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_w(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_e(elf, elves) {
            return Some(elf_move);
        }
        return None;
    } else if round % 4 == 1 {
        if let Some(elf_move) = consider_s(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_w(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_e(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_n(elf, elves) {
            return Some(elf_move);
        }
        return None;
    } else if round % 4 == 2 {
        if let Some(elf_move) = consider_w(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_e(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_n(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_s(elf, elves) {
            return Some(elf_move);
        }
        return None;
    } else if round % 4 == 3 {
        if let Some(elf_move) = consider_e(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_n(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_s(elf, elves) {
            return Some(elf_move);
        }
        if let Some(elf_move) = consider_w(elf, elves) {
            return Some(elf_move);
        }
        return None;
    }

    panic!("what?");
}

pub fn any_adjacent_elves(position: &Position, elves: &HashSet<Position>) -> bool {
    
    let left = Position { x: position.x - 1, y: position.y };
    if elves.contains(&left) {
        return true;
    }
    let right = Position { x: position.x + 1, y: position.y };
    if elves.contains(&right) {
        return true;
    }
    let up = Position { x: position.x, y: position.y - 1 };
    if elves.contains(&up) {
        return true;
    }
    let down = Position { x: position.x, y: position.y + 1 };
    if elves.contains(&down) {
        return true;
    }
    let up_left = Position { x: position.x - 1, y: position.y - 1 };
    if elves.contains(&up_left) {
        return true;
    }
    let up_right = Position { x: position.x + 1, y: position.y - 1 };
    if elves.contains(&up_right) {
        return true;
    }
    let down_left = Position { x: position.x - 1, y: position.y + 1 };
    if elves.contains(&down_left) {
        return true;
    }
    let down_right = Position { x: position.x + 1, y: position.y + 1 };
    if elves.contains(&down_right) {
        return true;
    }
    false
}

pub fn part_1(input: &str) -> String {
    let mut elves = parse_input(input);
    /*print_grid(&elves);
    println!();*/
    
    for round in 0..10 {
        let mut moves: Vec<ElfMove> = Vec::new();
        let mut to_spots: HashMap<Position, usize> = HashMap::new();
        for elf in elves.iter() {
            if any_adjacent_elves(elf, &elves) {
                let proposed = propose_move(elf, &elves, round);
                if let Some(proposed_move) = proposed {
                    let e = to_spots.entry(proposed_move.to.clone()).or_insert(0);
                    *e += 1;
                    moves.push(proposed_move);
                }
            }
        }
        let mut good_moves: Vec<ElfMove> = Vec::new();
        for m in moves.into_iter() {
            //println!("proposed move: {}", m);
            if *to_spots.get(&m.to).unwrap() == 1 {
                good_moves.push(m);
            }
        }

        for m in good_moves.iter() {
            elves.remove(&m.from);
        }
        for m in good_moves.into_iter() {
            elves.insert(m.to);
        }

        /*println!("== End of Round {} ==", round + 1);
        print_grid(&elves);
        println!();*/
    }

    let (max_x, max_y, min_x, min_y) = max_xy_min_xy(&elves);
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let cells = width * height;
    println!("width:{}, height:{}, cells:{}", width, height, cells);
    let empties = cells - elves.len() as isize;
    empties.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut elves = parse_input(input);
    let mut round = 0;
    let mut elf_moved = true; 
    while elf_moved {
        let mut moves: Vec<ElfMove> = Vec::new();
        let mut to_spots: HashMap<Position, usize> = HashMap::new();
        for elf in elves.iter() {
            if any_adjacent_elves(elf, &elves) {
                let proposed = propose_move(elf, &elves, round);
                if let Some(proposed_move) = proposed {
                    let e = to_spots.entry(proposed_move.to.clone()).or_insert(0);
                    *e += 1;
                    moves.push(proposed_move);
                }
            }
        }
        let mut good_moves: Vec<ElfMove> = Vec::new();
        for m in moves.into_iter() {
            //println!("proposed move: {}", m);
            if *to_spots.get(&m.to).unwrap() == 1 {
                good_moves.push(m);
            }
        }

        if good_moves.len() > 0 {
            for m in good_moves.iter() {
                elves.remove(&m.from);
            }
            for m in good_moves.into_iter() {
                elves.insert(m.to);
            }
            round += 1;
        } else {
            elf_moved = false;
        }
    }

    (round + 1).to_string()
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
        assert_eq!(result, "110");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "4123");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "20");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "1029");
	}
}
