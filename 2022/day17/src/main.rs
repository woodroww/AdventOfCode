use std::collections::HashMap;
use colored::*;

#[derive(Clone)]
struct Shape {
    width: usize,
    height: usize,
    bottom: usize,
    left: usize,
    shape: Vec<Vec<bool>>,
}

impl Shape {

    fn move_down(&mut self) {
        self.bottom -= 1;
    }

    fn hits_floor(&self, floor: &StopLine) -> bool {
        self.bottom == 0
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

    /*
    for s in ["-", "+", "|", "s", "L"] {
        let mut shape = spawn_shape(s, &shapes);
        shape.bottom = floor.height() + 4;
        println!("shape: {}", s);
        floor.print_floor(&shape);
        println!();
    }
    */

    let mut shape = spawn_shape("-", &shapes);
    shape.bottom = floor.height() + 4;
    let mut shape_moving = true;
    let mut count = 0;
    while shape_moving && count < 5 {
        shape.move_down();
        if shape.hits_floor(&floor) {
            shape_moving = false;
        }
        floor.print_floor(&shape);
        count += 1;
    }

    //shape.bottom -= 1;

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
