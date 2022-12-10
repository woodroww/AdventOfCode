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

        if let Some(pos) = adjust_tail(&tail_pos, &head_pos, None) {
            tail_pos = pos.clone();
            tail_visits.insert(pos.clone());
        }
    }

    tail_visits.iter().count()
}

fn part_2(input: String) -> usize {
    let mut tail_visits: HashSet<Position> = HashSet::new();
    // head H is rope[0]
    // tail 9 is rope[9]
    let mut rope = vec![Position { x: 0, y: 0 }; 10];

    tail_visits.insert(rope[9].clone());

    let end = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(_direction, count)| count.parse::<usize>().unwrap())
        .flat_map(|count| vec![0; count])
        .count();
    let inspect_move = (0..end).collect::<Vec<usize>>();

 //   let inspect_move = (5..14).collect::<Vec<usize>>();

    for (move_idx, direction) in input
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
        let extra_rope = rope.clone();
        let windowed = rope.iter_mut().sliding_windows(&mut storage);

        for (i, mut head_tail) in windowed.enumerate() {
            let mut rope_parts = head_tail.iter_mut();
            let head = rope_parts.next().unwrap();
            let tail = rope_parts.next().unwrap();
            /*if inspect_move.contains(&move_idx) {
                if let Some(pos) = adjust_tail(tail, head, Some(&extra_rope)) {
                    **tail = pos;
                }
            } else {*/
            if let Some(pos) = adjust_tail(tail, head, None) {
                **tail = pos;
            }
            //}
        }

        tail_visits.insert(rope[9].clone());

            /*
        if inspect_move.contains(&move_idx) {
            println!("{} {}", move_idx + 1, direction);
            print_rope(&rope);
            println!();
        }*/
    }

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

fn adjust_tail(tail: &Position, head: &Position, rope: Option<&Vec<Position>>) -> Option<Position> {

    let mut print_adjust = false; 
    if let Some(rope) = rope {
        print_adjust = true;
        print!("head:{} tail:{}", head, tail);
        print!(" diff: ({} {})", head.x - tail.x, head.y - tail.y);
    }

    let mut new_tail = Position {
        x: tail.x,
        y: tail.y,
    };
    if tail.x == head.x {
        if tail.y - head.y > 1 {
            new_tail.y -= 1;
        } else if head.y - tail.y > 1 {
            new_tail.y += 1;
        }
    } else if tail.y == head.y {
        if tail.x - head.x > 1 {
            new_tail.x -= 1;
        } else if head.x - tail.x > 1 {
            new_tail.x += 1;
        }
    } else {

        if (tail.y - head.y).abs() > 1 && (tail.x - head.x).abs() > 1 {
            if tail.y - head.y > 1 {
                if print_adjust == true {
                    print!(" double diag A jam xdiff:{} ydiff{}", head.x - tail.x, head.y - tail.y);
                }
                new_tail.y -= 1;
            }
            if head.y - tail.y > 1 {
                if print_adjust == true {
                    print!(" double diag B jam xdiff:{} ydiff{}", head.x - tail.x, head.y - tail.y);
                }
                new_tail.y += 1;
            }
            if tail.x - head.x > 1 {
                new_tail.x -= 1;
            }
            if head.x - tail.x > 1 {
                new_tail.x += 1;
            }


        } else if (tail.y - head.y).abs() > 1 {
            if tail.y - head.y > 1 {
                if print_adjust == true {
                    print!(" diag A xdiff{} ydiff{}", head.x - tail.x, head.y - tail.y);
                }
                new_tail.x += head.x - tail.x;
                new_tail.y += (head.y - tail.y) + 1;
            }
            if head.y - tail.y > 1 {
                if print_adjust == true {
                    print!(" diag B xdiff{} ydiff{}", head.x - tail.x, head.y - tail.y);
                }
                new_tail.x += head.x - tail.x;
                new_tail.y += (head.y - tail.y) - 1;
            }
        } else if (tail.x - head.x).abs() > 1 {
            if tail.x - head.x > 1 {
                if print_adjust == true {
                    print!(" diag C xdiff{} ydiff{}", head.x - tail.x, head.y - tail.y);
                }
                new_tail.x += (head.x - tail.x) + 1;
                new_tail.y += head.y - tail.y;
            }
            if head.x - tail.x > 1 {
                if print_adjust == true {
                    print!(" diag D xdiff{} ydiff{}", head.x - tail.x, head.y - tail.y);
                }
                new_tail.x += (head.x - tail.x) - 1;
                new_tail.y += head.y - tail.y;
            }
        }
    }

    if tail.x != new_tail.x || tail.y != new_tail.y {
        if print_adjust {
            println!(" new tail:{}", new_tail);
        }
        Some(new_tail)
    } else {
        if print_adjust {
            println!();
        }
        None
    }
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
