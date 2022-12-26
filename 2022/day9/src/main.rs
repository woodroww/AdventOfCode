use std::collections::HashSet;

use sliding_windows::IterExt;
use sliding_windows::Storage;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct GridSize {
    x: isize,
    x_origin: isize,
    y: isize,
    y_origin: isize,
}


fn part_1(input: String) -> usize {
    let mut tail_visits: HashSet<Position> = HashSet::new();
    let mut tail_pos = Position { x: 0, y: 0 };
    let mut head_pos = Position { x: 0, y: 0 };

    tail_visits.insert(tail_pos.clone());

    for direction in input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(direction, count)| (direction, count.parse::<usize>().unwrap()))
        .flat_map(|(direction, count)| vec![direction; count])
    {
        match direction {
            "U" => head_pos.y += 1,
            "D" => head_pos.y -= 1,
            "L" => head_pos.x -= 1,
            "R" => head_pos.x += 1,
            _ => {
                panic!("something wrong with input");
            }
        };

        if adjust_tail(&mut tail_pos, &head_pos) {
            tail_visits.insert(tail_pos.clone());
        }
    }

    tail_visits.iter().count()
}

fn total_move_count(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(_direction, count)| count.parse::<usize>().unwrap())
        .fold(0, |acc, count| acc + count)
}

fn grid_size(input: &str) -> GridSize {
    let mut pos = Position { x: 0, y: 0 };
    let mut max_pos = Position { x: 0, y: 0 }; 
    let mut min_pos = Position { x: 0, y: 0 }; 
    for (direction, count) in input.lines().map(|line| line.split_once(" ").unwrap()) {
        match direction {
            "U" => {
                pos.y += count.parse::<isize>().unwrap();
                if pos.y > max_pos.y {
                    max_pos.y = pos.y;
                }
            }
            "D" => {
                pos.y -= count.parse::<isize>().unwrap();
                if pos.y < min_pos.y {
                    min_pos.y = pos.y;
                }
            }
            "L" => {
                pos.x -= count.parse::<isize>().unwrap();
                if pos.x < min_pos.x {
                    min_pos.x = pos.x;
                }
            }
            "R" => {
                pos.x += count.parse::<isize>().unwrap();
                if pos.x > max_pos.x {
                    max_pos.x = pos.x;
                }
            }
            _ => unreachable!(),
        }
    }
    
    GridSize {
        x: max_pos.x - min_pos.x,
        x_origin: -min_pos.x,
        y: max_pos.y - min_pos.y,
        y_origin: -min_pos.y,
    }
}

fn part_2(input: String) -> usize {
    // where has the tail been
    let mut tail_visits: HashSet<Position> = HashSet::new();
    // head H is rope[0], tail 9 is rope[9]
    let mut rope = vec![Position { x: 0, y: 0 }; 10];

    // the tail starting position counts
    tail_visits.insert(rope[9].clone());

    //println!("total moves {}", total_move_count(&input));
    println!("grid_size {:?}", grid_size(&input));
    //let inspect_move = (0..total_move_count(&input)).collect::<Vec<usize>>();
    //let inspect_move = (5..14).collect::<Vec<usize>>();

    for (_move_idx, direction) in input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(direction, count)| (direction, count.parse::<usize>().unwrap()))
        .flat_map(|(direction, count)| vec![direction; count])
        .enumerate()
    {
        match direction {
            "U" => rope[0].y += 1,
            "D" => rope[0].y -= 1,
            "L" => rope[0].x -= 1,
            "R" => rope[0].x += 1,
            _ => {
                panic!("something wrong with input");
            }
        };

        let mut storage: Storage<&mut Position> = Storage::new(2);
        let windowed = rope.iter_mut().sliding_windows(&mut storage);

        for (_i, mut head_tail) in windowed.enumerate() {
            let mut rope_parts = head_tail.iter_mut();
            let head = rope_parts.next().unwrap();
            let tail = rope_parts.next().unwrap();
            adjust_tail(tail, head);
        }
        // print_rope(&rope);
        tail_visits.insert(rope[9].clone());
    }

    println!("final head {}", rope[0]);
    tail_visits.iter().count()
}

fn print_grid(head: &Position, tail: &Position, size_x: isize, size_y: isize) {
    for y in (0..=size_y).rev() {
        for x in 0..=size_x {
            if head.x == x && head.y == y {
                print!("H");
            } else if tail.x == x && tail.y == y {
                print!("T");
            } else if y == 0 && x == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

// cleaned up logic thanks to:
// https://www.youtube.com/watch?v=D-ce_rFtfD8

fn adjust_tail(tail: &mut Position, head: &Position) -> bool {
    let x_diff = head.x - tail.x;
    let y_diff = head.y - tail.y;
    let mut tail_modified = false;

    if x_diff.abs() >= 2 && y_diff.abs() >= 2 {
        if tail.x < head.x {
            tail.x = head.x - 1;
        } else {
            tail.x = head.x + 1;
        }
        if tail.y < head.y {
            tail.y = head.y - 1;
        } else {
            tail.y = head.y + 1;
        }
        tail_modified = true;
    } else if x_diff.abs() >= 2 {
        if tail.x < head.x {
            tail.x = head.x - 1;
        } else {
            tail.x = head.x + 1;
        }
        tail.y = head.y;
        tail_modified = true;
    } else if y_diff.abs() >= 2 {
        tail.x = head.x;
        if tail.y < head.y {
            tail.y = head.y - 1;
        } else {
            tail.y = head.y + 1;
        }
        tail_modified = true;
    }

    tail_modified
}

fn print_rope(rope: &Vec<Position>) {
    for (knot, head) in rope.iter().enumerate() {
        if knot == 0 {
            print!("H{} ", head);
        } else {
            print!("{}{} ", knot, head);
        }
    }
    println!();
    //let size_y = 5;
    //let size_x = 6;
    let origin_x = 11;
    let origin_y = 5;

    let size_y = 21;
    let size_x = 26;
    for y in (0..size_y).rev() {
        for x in 0..size_x {
            let it = rope
                .iter()
                .enumerate()
                .map(|(i, pos)| {
                    (
                        i,
                        Position {
                            x: pos.x + origin_x,
                            y: pos.y + origin_y,
                        },
                    )
                })
                .filter(|(_i, pos)| {
                    if pos.x == x && pos.y == y {
                        true
                    } else {
                        false
                    }
                });
            let min_knot = it.clone().map(|(i, _pos)| i).min();
            let mut printed = false;
            if let Some(min) = min_knot {
                if min == 0 {
                    print!("H");
                } else {
                    print!("{}", min);
                }
                printed = true;
            }
            if printed == false {
                print!(".");
            }
        }
        println!();
    }
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
    //println!("Part 1: {}", part_1(input.clone()));
    //println!("Part 2: {}", part_2(input));

    /*println!(
        "big: {}",
        part_2(std::fs::read_to_string("big_example.txt").expect("No big example"))
    );*/
    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(input.clone())); // 9257 too high
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(input);
        assert!(result == 13);
    }

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(input);
        assert!(result == 6498);
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(input);
        assert_eq!(result, 2531);
    }
}
