use std::collections::HashMap;
use colored::*;

#[derive(Clone, Debug)]
struct Shape {
    width: usize,
    height: usize,
    bottom: usize,
    left: usize,
    data: Vec<Vec<bool>>,
}

const LAST_INDEX_RIGHT: usize = 6;

impl Shape {

    fn move_down(&mut self) {
        self.bottom -= 1;
    }

    fn right(&self) -> usize {
        self.left + (self.width - 1)
    }

    fn take_push(&mut self, dir: char) -> bool { // bool moved if true
        let before = self.left;
        match dir {
            '>' => {
                if self.right() < LAST_INDEX_RIGHT {
                    self.left += 1;
                }
            }
            '<' => {
                if self.left > 0 {
                    self.left -= 1;
                }
            }
            _ => {
                panic!("invalid char in input")
            }
        }
        before != self.left
    }

    fn hits_floor(&self, floor: &StopLine) -> bool {
        let floor_height = *floor.line.iter().max().unwrap();
        if self.bottom <= floor_height + 1 {
            for (i, floor_height) in floor.line.iter().enumerate() {
                if i >= self.left && i <= self.left + (self.width - 1) {

                    if self.bottom == 0 {
                        panic!("{:#?}\nfloor_height:{}", self, floor_height);
                    }
                    let shape_row = 0;
                    let shape_col = i - self.left;
                    if shape_row >= self.data.len() {
                        panic!("{:#?}\nfloor_height:{}", self, floor_height);
                    }
                    if shape_col >= self.data[shape_row].len() {
                        panic!("{:#?}\nfloor_height:{}", self, floor_height);
                    }

                    if self.data[shape_row][shape_col] == true {
                        return true;
                    }
                }
            }
            false
        } else {
            false
        }
    }

    fn contains_point(&self, x: usize, y: usize) -> bool {
        if x >= self.left && x <= self.left + (self.width - 1) {
            if y >= self.bottom && y < self.bottom + self.height {
                //println!("{} y pass test", y);
                let col = x - self.left;
                let row = y - self.bottom;
                if self.data[row][col] == true {
                    return true;
                }
            }
        }
        false
    }

//  |..@@@@.|
    fn h_line() -> Self {
        let mut shape = Vec::new();
        shape.push(vec![true, true, true, true]);
        let height = 1;
        Shape {
            width: 4,
            height,
            bottom: 0 + (height - 1),
            left: 2,
            data: shape,
        }
    }

//  |...@...|
//  |..@@@..|
//  |...@...|
    fn plus() -> Self {
        let mut shape = Vec::new();
        shape.push(vec![false, true, false]);
        shape.push(vec![true, true, true]);
        shape.push(vec![false, true, false]);
        let height = 3;
        Shape {
            width: 3,
            height,
            bottom: 0 + (height - 1),
            left: 2,
            data: shape,
        }
    }

//  |....@..|
//  |....@..|
//  |..@@@..|
    fn l_shape() -> Self {
        let mut shape = Vec::new();
        shape.push(vec![true, true, true]);
        shape.push(vec![false, false, true]);
        shape.push(vec![false, false, true]);
        let height = 3;
        Shape {
            width: 3,
            height,
            bottom: 0 + (height - 1),
            left: 2,
            data: shape,
        }
    }

//  |..@....|
//  |..@....|
//  |..@....|
//  |..@....|
    fn v_line() -> Self {
        let mut shape = Vec::new();
        shape.push(vec![true]);
        shape.push(vec![true]);
        shape.push(vec![true]);
        shape.push(vec![true]);
        let height = 4;
        Shape {
            width: 1,
            height: 4,
            bottom: 0 + (height - 1),
            left: 2,
            data: shape,
        }
    }

//  |..@@...|
//  |..@@...|
    fn square() -> Self {
        let mut shape = Vec::new();
        shape.push(vec![true, true]);
        shape.push(vec![true, true]);
        let height = 2;
        Shape {
            width: 2,
            height: 2,
            bottom: 0 + (height - 1),
            left: 2,
            data: shape,
        }
    }
}

/*
// cave is 7 units wide .......
|..@@@@.|
|.......|   each rock appears with its bottom 3 unit above hightest rock or floor if no rocks
|.......|
|.......|
+-------+
*/

struct StopLine {
    line: [usize; 7],
}

impl StopLine {
    fn new() -> Self {
        Self { line: [0; 7] }
    }
    fn height(&self) -> usize {
        *self.line.iter().max().unwrap()
    }

    fn absorb_rock(&mut self, shape: &Shape) {
        for (i, col) in self.line.iter_mut().enumerate() {
            //println!("left:{}, right:{}", shape.left, shape.right());
            if i >= shape.left && i <= shape.right() {
                let shape_col = i - shape.left;
                let shape_row_count = shape.data.len();
                let mut highest = 0;
                for (r, shape_line) in shape.data.iter().rev().enumerate() {
                    if shape_line[shape_col] == true {
                        if highest < r + 1 {
                            highest = r + 1;
                        }
                    }
                }
                *col += highest;
            }
        }
    }

    fn print_floor(&self, shape: &Shape) {
        let max_height = self.line.iter().max().unwrap();
        let last_draw_line = shape.bottom + shape.height;
        for row in (0..last_draw_line).rev() { // plus 1 to draw floor at 0
            if row < 10 {
                print!("{}  ", row);
            } else {
                print!("{} ", row);
            }
            if row == 0 {
                print!("{}", "+".blue());
            } else {
                print!("{}", "|".blue());
            }
            for col in 0..self.line.len() {
                if shape.contains_point(col, row) {
                    print!("{}", format!("{}", "@".yellow()));
                } else if self.line[col] >= row {
                    if row == 0 {
                        print!("{}", "-".blue());
                    } else {
                        print!("{}", "#".red());
                    }
                } else {
                    print!("{}", ".".blue());
                }
            }
            if row == 0 {
                print!("{}", "+".blue());
            } else {
                print!("{}", "|".blue());
            }
            println!(" {}", last_draw_line - row - 1);
        }
    }
}

fn make_shapes() -> HashMap<String, Shape> {
    let mut map = HashMap::new();
    map.insert("-".to_string(), Shape::h_line());
    map.insert("+".to_string(), Shape::plus());
    map.insert("L".to_string(), Shape::l_shape());
    map.insert("|".to_string(), Shape::v_line());
    map.insert("s".to_string(), Shape::square());
    map
}

fn spawn_shape(key: &str, map: &HashMap<String, Shape>) -> Shape {
    map.get(key).unwrap().clone()
}

    /*for s in ["-", "+", "L", "|", "s"] {
        let mut shape = spawn_shape(s, &shapes);
        shape.bottom = floor.height() + 4;
        println!("shape: {}", s);

        let mut shape_moving = true;
        let mut count = 0;
        while shape_moving && count < 5 {
            if shape.hits_floor(&floor) {
                shape_moving = false;
            } else {
                shape.move_down();
            }
            floor.print_floor(&shape);
            count += 1;
        }
    }*/

fn part_1(input: &str) -> String {
    let mut floor = StopLine::new();
    let shapes = make_shapes();
    let mut pushes = input.trim_end().chars().peekable();

    for s in ["-", "+", "L", "|", "s"].iter().cycle() {
        let mut shape = spawn_shape(s, &shapes);
        shape.bottom = floor.height() + 4;

        println!("\nA new rock begins falling:");
        floor.print_floor(&shape);

        while let Some(push) = pushes.next() {
            let moved;
            match push {
                '>' => {
                    print!("\nJet of gas pushes rock right");
                    moved = shape.take_push(push);
                }
                '<' => {
                    print!("\nJet of gas pushes rock left");
                    moved = shape.take_push(push);
                }
                _ => panic!("invalid char in input"),
            }
            if moved {
                println!(":");
            } else {
                println!(", but nothing happens:");
            }
            floor.print_floor(&shape);
            if !shape.hits_floor(&floor) {
                shape.move_down();
                println!("\nRock falls 1 unit:");
                floor.print_floor(&shape);
            } else {
                floor.absorb_rock(&shape);
                break;
            }
        }
        println!("\nRock falls 1 unit, causing it to come to rest:");
        floor.print_floor(&shape);
        if pushes.peek().is_none() {
            break;
        }
    }

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
        assert_eq!(result, "0");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}
}
