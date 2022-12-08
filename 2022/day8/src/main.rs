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

fn is_visible(
    x_idx: usize,
    y_idx: usize,
    grid: &Vec<Vec<u8>>,
    y_size: usize,
    x_size: usize,
) -> bool {
    // return true if visible from any direction
    let value = grid[y_idx][x_idx];

    let mut left = (0..x_idx).map(|i| (y_idx, i));
    let mut right = (x_idx + 1..x_size).map(|i| (y_idx, i));
    let mut up = (0..y_idx).map(|i| (i, x_idx));
    let mut down = (y_idx + 1..y_size).map(|i| (i, x_idx));

    if left.find(|(y, x)| grid[*y][*x] >= value).is_none()
        || right.find(|(y, x)| grid[*y][*x] >= value).is_none()
        || up.find(|(y, x)| grid[*y][*x] >= value).is_none()
        || down.find(|(y, x)| grid[*y][*x] >= value).is_none()
    {
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

    let up = up_iter
        .position(|grid_value| grid_value >= value)
        .unwrap_or(y_idx - 1)
        + 1;
    let left = left_iter
        .position(|grid_value| grid_value >= value)
        .unwrap_or(x_idx - 1)
        + 1;
    let right = right_iter
        .position(|grid_value| grid_value >= value)
        .unwrap_or(x_size - (x_idx + 1) - 1)
        + 1;
    let down = down_iter
        .position(|grid_value| grid_value >= value)
        .unwrap_or(y_size - (y_idx + 1) - 1)
        + 1;

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
    // add inner count to the number of outside trees
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
    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let answer = 21;

        let input = input_txt(InputFile::Example);
        let result = part_1(input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_example_part_2() {
        let answer = 8;

        let input = input_txt(InputFile::Example);
        let result = part_2(input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_real_part_1() {
        let answer = 1792;

        let input = input_txt(InputFile::Real);
        let result = part_1(input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_real_part_2() {
        let answer = 334880;

        let input = input_txt(InputFile::Real);
        let result = part_2(input);
        assert_eq!(result, answer);
    }
}
