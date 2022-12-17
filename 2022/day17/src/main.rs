use std::collections::HashMap;
use colored::*;

#[derive(Clone, Debug)]
struct Shape {
    width: usize,
    height: usize,
    bottom: usize,
    left: usize,
    shape: Vec<Vec<bool>>,
}

const LAST_INDEX_RIGHT: usize = 6;

impl Shape {

    fn move_down(&mut self) {
        self.bottom -= 1;
    }

    fn right(&self) -> usize {
        self.left + (self.width - 1)
    }

    fn take_push(&mut self, dir: char) {
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
    }

    fn hits_floor(&self, floor: &StopLine) -> bool {
        let floor_height = *floor.line.iter().max().unwrap();
        if self.bottom <= floor_height + 1 {
            for (i, floor_height) in floor.line.iter().enumerate() {
                if i >= self.left && i <= self.left + (self.width - 1) {

                    if self.bottom == 0 {
                        panic!("{:#?}\nfloor_height:{}", self, floor_height);
                    }
                    if self.shape[self.bottom - 1][i - self.left] == true {
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
                if self.shape[row][col] == true {
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
            shape,
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
            shape,
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
            shape,
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
            shape,
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
            shape,
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
    /*
+-------+
|..####.|

+-------+
|..####.|
|...@...|
|..@@@..|
|...@...|
    */
    fn fit_shape(&mut self, shape: &Shape) {
        // if shape.bottom (which is really the top) is 1 away, or next to
        // check to see where the shape can move
        if shape.bottom - self.line.iter().max().unwrap() == 1 {
        }
    }

    fn print_floor(&self, shape: &Shape) {
        let floor: [usize; 7] = [0, 0, 0, 0, 0, 0, 0];
        let max_height = floor.iter().max().unwrap();
        let last_draw_line = 8;
        for row in (0..last_draw_line).rev() { // plus 1 to draw floor at 0
            if row < 10 {
                print!("{}  ", row);
            } else {
                print!("{} ", row);
            }
            for col in 0..floor.len() {
                if shape.contains_point(col, row) {
                    print!("{}", format!("{}", "#".yellow()));
                } else if floor[col] >= row {
                    print!("{}", format!("{}", "#".red()));
                } else {
                    print!("{}", format!("{}", ".".blue()));
                }
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

fn part_1(input: &str) -> String {


    let floor = StopLine::new();
    let shapes = make_shapes();

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

    let mut pushes = input.trim_end().chars().peekable();

    for s in ["-", "+", "L", "|", "s"].iter().cycle() {
        let mut shape = spawn_shape(s, &shapes);
        shape.bottom = floor.height() + 4;

        println!("\nA new rock begins falling:");
        floor.print_floor(&shape);

        while let Some(push) = pushes.next() {
            match push {
                '>' => {
                    println!("\nJet of gas pushes rock right:");
                    shape.take_push(push);
                }
                '<' => {
                    println!("\nJet of gas pushes rock left:");
                    shape.take_push(push);
                }
                _ => panic!("invalid char in input"),
            }
            floor.print_floor(&shape);
            if !shape.hits_floor(&floor) {
                shape.move_down();
                println!("\nRock falls 1 unit:");
                floor.print_floor(&shape);
            } else {
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
