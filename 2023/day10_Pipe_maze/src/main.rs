/*

╭─╮
│ │
╰─╯

│     | is a vertical pipe connecting north and south.
─     - is a horizontal pipe connecting east and west.
╰     L is a 90-degree bend connecting north and east.
╯     J is a 90-degree bend connecting north and west.
╮     7 is a 90-degree bend connecting south and west.
╭     F is a 90-degree bend connecting south and east.
.     . is ground; there is no pipe in this tile.
S     S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.



*/

use std::collections::HashSet;
use colored::Colorize;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
enum PipeType {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl std::fmt::Display for PipeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PipeType::NS => '│',
                PipeType::EW => '─',
                PipeType::NE => '╰',
                PipeType::NW => '╯',
                PipeType::SW => '╮',
                PipeType::SE => '╭',
                PipeType::Ground => '.',
                PipeType::Start => 'S',
            }
        )
    }
}

fn parse_input(input: &str) -> (Vec<Vec<PipeType>>, (usize, usize)) {
    let mut rows = vec![];
    let (mut start_row, mut start_col) = (0, 0);
    for (row, line) in input.lines().enumerate() {
        let mut columns = vec![];
        for (col, c) in line.chars().enumerate() {
            columns.push(match c {
                '|' => PipeType::NS,
                '-' => PipeType::EW,
                'L' => PipeType::NE,
                'J' => PipeType::NW,
                '7' => PipeType::SW,
                'F' => PipeType::SE,
                '.' => PipeType::Ground,
                'S' => {
                    start_row = row;
                    start_col = col;
                    PipeType::Start
                }
                _ => panic!("illegal char"),
            });
        }
        rows.push(columns);
    }
    (rows, (start_row, start_col))
}

fn print_pipes(pipes: &Vec<Vec<PipeType>>, main_loop: Option<&Vec<(usize, usize)>>) {
    match main_loop {
        Some(ml) => {
            for (r, row) in pipes.iter().enumerate() {
                for (c, pipe) in row.iter().enumerate() {
                    if ml.contains(&(r, c)) {
                        print!("{}", format!("{}", pipe).red());
                    } else {
                        print!("{}", pipe);
                    }
                }
                println!();
            }
        }
        None => {
            for row in pipes {
                for pipe in row {
                    print!("{}", pipe);
                }
                println!();
            }
        }
    }
}

fn acceptable_neighbors(col: usize, row: usize, pipes: &Vec<Vec<PipeType>>) -> Vec<Neighbor> {
    let mut results = vec![];
    let neighbors = neighbors(col, row, pipes.first().unwrap().len(), pipes.len());
    //println!("neighbors {:?}", neighbors);
    for (n_row, n_col) in  neighbors {
        if pipes[n_row][n_col] != PipeType::Ground {
            let mut acceptable_pipes = vec![];
            match pipes[row][col] {
                PipeType::NS => {
                    if n_row > row {
                    // PipeType::NS => '│', NS => '│',NE => '╰',NW => '╯',
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
                    } else if n_row < row {
                    // PipeType::NS => '│', NS => '│',SE => '╭',SW => '╮',
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::SE, PipeType::SW]);
                    }
                },
                PipeType::EW => {
                    if n_col > col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
                    } else if n_col < col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
                    }
                }
                PipeType::NE => {
                    // NE => '╰', NS => '│', SE => '╭', SW => '╮',
                    if n_row < row {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::SE, PipeType::SW]);
                    }
                    // NE => '╰', EW => '─', NW => '╯', SW => '╮',
                    if n_col > col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
                    } 
                }
                PipeType::NW => {
                    if n_row < row {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::SE, PipeType::SW]);
                    }
                    if n_col < col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
                    }
                }
                PipeType::SW => {
                    if n_row > row {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
                    }
                    if n_col < col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
                    }
                }
                PipeType::SE => {
                    if n_row > row {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
                    }
                    if n_col > col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
                    }
                }
                PipeType::Ground => {
                }
                PipeType::Start => {
                    if n_row > row {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
                    } else if n_row < row {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::SE, PipeType::SW]);
                    }
                    if n_col > col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
                    } else if n_col < col {
                        acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
                    }
                }
            }

            results.push(Neighbor {
                acceptable: acceptable_pipes,
                row: n_row,
                col: n_col,
            });
        }
    }
    results
}

            /*
            if n_row > row {
                acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
            } else if n_row < row {
                acceptable_pipes.extend_from_slice(&vec![PipeType::NS, PipeType::SE, PipeType::SW]);
            }
            if n_col > col {
                acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
            } else if n_col < col {
                acceptable_pipes.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
            }
        */
fn calc_start_pipe(
    pipes: &Vec<Vec<PipeType>>,
    start_row: usize,
    start_col: usize,
) -> PipeType {
    //println!("calc_start_pipe");
    let node = connections(start_col, start_row, &pipes);
    //println!("start node {}", node);

    let row = start_row;
    let col = start_col;
    let n_row = node.forward.row;
    let n_col = node.forward.col;
    let mut acceptable_forward = vec![];

    if n_row > start_row {
        acceptable_forward.extend_from_slice(&vec![PipeType::NS, PipeType::SW, PipeType::SE]);
    } else if n_row < start_row {
        acceptable_forward.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
    }
    if n_col > start_col {
        acceptable_forward.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
    } else if n_col < start_col {
        acceptable_forward.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
    }

    let n_row = node.backward.row;
    let n_col = node.backward.col;
    let mut acceptable_backward = vec![];

    if n_row > start_row {
        acceptable_backward.extend_from_slice(&vec![PipeType::NS, PipeType::SW, PipeType::SE]);
    } else if n_row < start_row {
        acceptable_backward.extend_from_slice(&vec![PipeType::NS, PipeType::NE, PipeType::NW]);
    }
    if n_col > start_col {
        acceptable_backward.extend_from_slice(&vec![PipeType::EW, PipeType::NE, PipeType::SE]);
    } else if n_col < start_col {
        acceptable_backward.extend_from_slice(&vec![PipeType::EW, PipeType::NW, PipeType::SW]);
    }
    /*
    print!("acceptable_backward: ");
    for pipe in &acceptable_backward {
        print!("{} ", pipe);
    }
    println!();
    print!("acceptable_forward:  ");
    for pipe in &acceptable_forward {
        print!("{} ", pipe);
    }
    println!();
    */

    let acceptable_backward: HashSet<PipeType> = acceptable_backward.into_iter().collect();
    let acceptable_forward: HashSet<PipeType> = acceptable_forward.into_iter().collect();
    let my_pipe = acceptable_forward.intersection(&acceptable_backward);
    let p = my_pipe.into_iter().next().unwrap();
    //println!("intersection {}", p);
    p.clone()
}

// must have the start position filled in with correct pipe
fn find_main_loop(
    pipes: &Vec<Vec<PipeType>>,
    start_row: usize,
    start_col: usize,
) -> Vec<(usize, usize)> {
    let mut row = start_row;
    let mut col = start_col;
    let mut prev: Option<(usize, usize)> = None;
    let mut main_loop = vec![];
    loop {
        if row == start_row && col == start_col && prev.is_some() {
            println!("break");
            break;
        }
        main_loop.push((row, col));
        println!("loop row:{} col:{}", row, col);
        let current_node = connections(col, row, &pipes);
        //println!("prev: {:?}", prev);
        println!("{}", current_node);
        println!();
        if let Some((prev_row, prev_col)) = prev {
            prev = Some((row, col));
            if current_node.forward.row == prev_row && current_node.forward.col == prev_col
            {
                row = current_node.backward.row;
                col = current_node.backward.col;
            } else {
                row = current_node.forward.row;
                col = current_node.forward.col;
            }
        } else {
            let start_node = connections(col, row, &pipes);
            prev = Some((row, col));
            row = start_node.forward.row;
            col = start_node.forward.col;
        }
    }
    main_loop
}

fn connections(col: usize, row: usize, pipes: &Vec<Vec<PipeType>>) -> Node {
    let acceptable = acceptable_neighbors(col, row, &pipes);
    print_pipes(pipes, None);
    let mut connected = vec![];
    for n in acceptable {
        print!("{} {} ", pipes[row][col], n);
        let real_pipe = &pipes[n.row][n.col];
        if n.acceptable.contains(&real_pipe) {
            connected.push(Connection {
                row: n.row,
                col: n.col,
                pipe: real_pipe.clone(),
            });
        }
        println!();
    }
    if connected.len() > 2 {
        println!("!! found MORE than two connections !!");
        for c in &connected {
            println!("connection: {}", c);
        }
        panic!();
    }
    if connected.len() < 2 {
        println!("!! found LESS than two connections !!");
        println!("row:{} col:{}", row, col);
        for c in &connected {
            println!("connection: {}", c);
        }
        panic!();
    }
    let mut i = connected.into_iter();
    Node {
        forward: i.next().unwrap(),
        backward: i.next().unwrap(),
    }
}

struct Connection {
    pipe: PipeType,
    row: usize,
    col: usize,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row:{} col:{} {}", self.row, self.col, self.pipe)
    }
}

struct Node {
    forward: Connection,
    backward: Connection,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "forward: {}\nbackward: {}", self.forward, self.backward)
    }
}

struct Neighbor {
    acceptable: Vec<PipeType>,
    row: usize,
    col: usize,
}

impl std::fmt::Display for Neighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "row:{} col:{} acceptable: ", self.row, self.col)?;
        for pipe in &self.acceptable {
            write!(f, "{:?}:{} ", pipe, pipe)?;
        }
        write!(f, "")
    }
}

// returns Vec<(row, col)>
pub fn neighbors(
    col: usize,
    row: usize,
    col_bound_exclusive: usize,
    row_bound_exclusive: usize,
) -> Vec<(usize, usize)> {
    let mut dirs = Vec::new();
    if let Some(x) = col.checked_sub(1) {
        dirs.push((row, x));
    }
    if let Some(y) = row.checked_sub(1) {
        dirs.push((y, col));
    }
    if let Some(x) = col.checked_add(1) {
        if x < col_bound_exclusive {
            dirs.push((row, x));
        }
    }
    if let Some(y) = row.checked_add(1) {
        if y < row_bound_exclusive {
            dirs.push((y, col));
        }
    }
    dirs
}

fn part_1(input: &str) -> String {
    let (mut pipes, (start_row, start_col)) = parse_input(input);
    //print_pipes(&pipes);
    let start_pipe = calc_start_pipe(&pipes, start_row, start_col);
    pipes[start_row][start_col] = start_pipe;
    //print_pipes(&pipes);
    let main_loop = find_main_loop(&pipes, start_row, start_col);
    print_pipes(&pipes, Some(&main_loop));


    println!("main loop {:?}", main_loop);

    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    //let input = std::fs::read_to_string("example2.txt").expect("No example.txt file");
    let input = std::fs::read_to_string("example3.txt").expect("No example.txt file");
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
