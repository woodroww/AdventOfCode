fn parse_grid(input: String) -> Vec<Vec<u8>> {
    let mut rows = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        rows.push(row);
    }
    rows
}

fn print_grid(grid: &Vec<Vec<u8>>) {
    for line in grid {
        for n in line {
            print!("{}", n);
        }
        println!();
    }
}

fn is_visible(
    x_idx: usize,
    y_idx: usize,
    grid: &Vec<Vec<u8>>,
    y_size: usize,
    x_size: usize,
) -> bool {
    // true if visible from any direction
    let value = grid[y_idx][x_idx];

    let l_hidden = (0..x_idx)
        .map(|i| (y_idx, i))
        .find(|(y, x)| grid[*y][*x] >= value)
        .is_some();
    if l_hidden == false {
        return true;
    }
    let r_hidden = (x_idx + 1..x_size)
        .map(|i| (y_idx, i))
        .find(|(y, x)| grid[*y][*x] >= value)
        .is_some();
    if r_hidden == false {
        return true;
    }
    let up_hidden = (0..y_idx)
        .map(|i| (i, x_idx))
        .find(|(y, x)| grid[*y][*x] >= value)
        .is_some();
    if up_hidden == false {
        return true;
    }
    let down_hidden = (y_idx + 1..y_size)
        .map(|i| (i, x_idx))
        .find(|(y, x)| grid[*y][*x] >= value)
        .is_some();
    if down_hidden == false {
        return true;
    }
    false
}

fn scenic_score(
    x_idx: usize,
    y_idx: usize,
    grid: &Vec<Vec<u8>>,
    y_size: usize,
    x_size: usize,
) -> usize {

    let value = grid[y_idx][x_idx];
    let mut up_iter = (0..y_idx).map(|i| grid[i][x_idx]).rev();
    let mut left_iter = (0..x_idx).map(|i| grid[y_idx][i]).rev();
    let mut right_iter = (x_idx + 1..x_size).map(|i| grid[y_idx][i]);
    let mut down_iter = (y_idx + 1..y_size).map(|i| grid[i][x_idx]);

//   println!("grid[{}][{}] {}", y_idx, x_idx, value);
    /*println!("up len {}", up_iter.len());
    println!("left len {}", left_iter.len());
    println!("right len {}", right_iter.len());
    println!("down len {}", down_iter.len());*/
    let up_len = up_iter.len() - 1;
    let left_len = left_iter.len() - 1;
    let right_len = right_iter.len() - 1;
    let down_len = down_iter.len() - 1;

    let up = up_iter.position(|grid_value| grid_value >= value).unwrap_or(up_len) + 1;
    let left = left_iter.position(|grid_value| grid_value >= value).unwrap_or(left_len) + 1;
    let right = right_iter.position(|grid_value| grid_value >= value).unwrap_or(right_len) + 1;
    let down = down_iter.position(|grid_value| grid_value >= value).unwrap_or(down_len) + 1;

    /*let mut count = 0;
    while let Some(a) = up_iter.next() {
        println!("up {}", a);
    }
    while let Some(a) = left_iter.next() {
        println!("left {}", a);
    }
    while let Some(a) = right_iter.next() {
        println!("right {}", a);
    }
    while let Some(a) = down_iter.next() {
        println!("down {}", a);
    }*/


/*
grid[1][2]
up 1
left 1
right 2
down 2
*/
/*
    println!("up {}", up);
    println!("left {}", left);
    println!("right {}", right);
    println!("down {}", down);
*/
    up * left * right * down
}

// What is the highest scenic score possible for any tree?
fn part_2(input: String) -> usize {
    let grid = parse_grid(input);
    let y_size = grid.iter().len();
    let x_size = grid.iter().nth(0).unwrap().len();
    let mut scores: Vec<usize> = Vec::new();

    for y in 1..y_size - 1 {
        for x in 1..x_size - 1 {
            scores.push(scenic_score(x, y, &grid, y_size, x_size));
        }
    }
    scores.into_iter().max().unwrap()
}

// how many trees are visible from outside the grid?
fn part_1(input: String) -> usize {
    let grid = parse_grid(input);
    //print_grid(&grid);
    let y_size = grid.iter().len();
    let x_size = grid.iter().nth(0).unwrap().len();
    let mut inner_count = 0;
    for y in 1..y_size - 1 {
        for x in 1..x_size - 1 {
            let vis = is_visible(x, y, &grid, y_size, x_size);
            if vis == true {
                inner_count += 1;
            }
        }
    }
    let total = (2 * y_size) + (2 * (x_size - 2)) + inner_count;
    total 
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
    let input = input_txt(InputFile::Real);
    //println!("Part 1: {}", part_1(input.clone()));
    //part_2(input);
    println!("Part 2: {}", part_2(input));

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let part_1_example_answer = 1;

        let input = input_txt(InputFile::Example);
        let result = part_1(input);
        assert_eq!(result, part_1_example_answer);
    }

    #[test]
    fn test_example_part_2() {
        let part_2_example_answer = 1;

        let input = input_txt(InputFile::Example);
        let result = part_2(input);
        assert_eq!(result, part_2_example_answer);
    }
}
