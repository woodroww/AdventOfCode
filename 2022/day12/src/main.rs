use std::collections::{HashSet, VecDeque};

fn char_to_value(c: u8) -> u8 {
    (if c < 91 { c - 38 } else { c - 96 }) as u8
}

pub fn cardinal_directions(
    x: usize,
    y: usize,
    x_bound: usize,
    y_bound: usize,
) -> Vec<(usize, usize)> {
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
    dirs
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 0, 0  left, top
struct Grid {
    cells: Vec<Vec<u8>>,
    start: Position,
    end: Position,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let (cells, start, end) = Self::make_grid(input);
        let start = start.unwrap();
        let end = end.unwrap();
        let height = cells.len();
        let width = cells.iter().nth(0).unwrap().len();
        Grid {
            cells,
            start: start.clone(),
            end,
            width,
            height,
        }
    }

    fn make_grid(input: &str) -> (Vec<Vec<u8>>, Option<Position>, Option<Position>) {
        let lines = input.lines().collect::<Vec<&str>>();
        let height = lines.len();
        let width = lines.iter().nth(0).unwrap().len();
        let mut grid: Vec<Vec<u8>> = vec![vec![0; width]; height];
        let mut start = None;
        let mut end = None;

        for (h, line) in lines.iter().enumerate() {
            for (w, c) in line.chars().enumerate() {
                if c == 'S' {
                    grid[h][w] = char_to_value('a' as u8);
                    start = Some(Position { x: w, y: h });
                } else if c == 'E' {
                    grid[h][w] = char_to_value('z' as u8);
                    end = Some(Position { x: w, y: h });
                } else {
                    grid[h][w] = char_to_value(c as u8);
                }
            }
        }
        (grid, start, end)
    }

    fn print_grid(&self, grid_type: GridDisplay) {
        match grid_type {
            GridDisplay::Letters => {
                for (y, line) in self.cells.iter().enumerate() {
                    for (x, c) in line.iter().enumerate() {
                        print!(" {}  ", (*c + 96) as char);
                    }
                    println!();
                }
            }
            GridDisplay::Numbers => {
                for (y, line) in self.cells.iter().enumerate() {
                    for (x, n) in line.iter().enumerate() {
                        if *n < 10 {
                            print!("  {}  ", n);
                        } else {
                            print!(" {}  ", n);
                        }
                    }
                    println!();
                }
            }
        }
        println!();
    }

    fn valid_moves_for_position(&self, position: Position) -> HashSet<Position> {
        let mut possible: HashSet<Position> = HashSet::new();
        let neighbors = cardinal_directions(position.x, position.y, self.width, self.height);
        let current_value = self.cells[position.y][position.x];

        for (x, y) in neighbors {
            let neighbor_value = self.cells[y][x];
            // can go one elevation higher or any elevation lower
            if neighbor_value > current_value {
                if neighbor_value - current_value == 1 {
                    possible.insert(Position { x, y });
                }
            } else {
                possible.insert(Position { x, y });
            }
        }
        possible
    }
}

// S starting position is elevation a
// E goal is elevation z

// can go one elevation higher or any elevation lower
// can move up, down, left, right

// what is least amount of steps to reach goal


enum GridDisplay {
    Letters,
    Numbers,
}


// a little help from https://youtu.be/DRODVXPgUcI

fn breadth_first_search(grid: &Grid, part_two: bool) -> String {
    // FIFO VecDeque
    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    
    if part_two {
        for (y, line) in grid.cells.iter().enumerate() {
            for (x, height) in line.iter().enumerate() {
                if *height == 1 {
                    queue.push_back((Position { x, y }, 0));
                }
            }
        }
    } else {
        queue.push_back((grid.start, 0));
    }

    let mut visited: HashSet<Position> = HashSet::new();

    while let Some(pos_count) = queue.pop_front() {
        if visited.contains(&pos_count.0) {
            continue;
        }
        visited.insert(pos_count.0);
        if pos_count.0 == grid.end {
            return pos_count.1.to_string();
        }
        let possible = grid.valid_moves_for_position(pos_count.0);
        for pos in possible {
            queue.push_back((pos, pos_count.1 + 1));
        }
    }

    "no path found".to_string()
}

fn part_1(input: &str) -> String {
    let grid = Grid::new(input);
    breadth_first_search(&grid, false)
}

fn part_2(input: &str) -> String {
    let grid = Grid::new(input);
    breadth_first_search(&grid, true)
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
    let input = input_txt(InputFile::Example);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "31");
    }

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "29");
    }

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "456");
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "454");
    }
}
