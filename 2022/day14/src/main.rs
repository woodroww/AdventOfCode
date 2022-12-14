#[derive(Clone, PartialEq, Eq, Hash)]
struct Position<T> {
    x: T,
    y: T,
}

impl<T> Position<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> std::fmt::Display for Position<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


fn max_xy_min_xy(cave: &Vec<Position<usize>>) -> (usize, usize, usize, usize) {
    let mut max_x = 0;
    let mut min_x = usize::MAX;
    let mut max_y = 0;
    let mut min_y = usize::MAX;

    for section in cave.iter() {
        if section.x > max_x {
            max_x = section.x;
        }
        if section.y > max_y {
            max_y = section.y;
        }
        if section.x < min_x {
            min_x = section.x;
        }
        if section.y < min_y {
            min_y = section.y;
        }
    }

    (max_x, max_y, min_x, min_y)
}

// a unit of sand is created at Position sand_spawner
// sand travels and when at rest next unit is created

enum CaveItem {
    Sand,
    Rock,
    Air
}

struct Cave {
    things: Vec<Vec<CaveItem>>,
}

impl Cave {
    fn new(rocks: Vec<Position<usize>>) -> Self {
        let (max_x, max_y, min_x, _min_y) = max_xy_min_xy(&rocks);
        let x_offset = min_x;
        //let sand_spawner = Position::new(500 - min_x, 0);
        let mut cave = Vec::new();
        for _ in 0..=max_y {
            let mut row: Vec<CaveItem> = Vec::new();
            for _ in 0..=max_x - min_x {
                row.push(CaveItem::Air);
            }
            cave.push(row);
        }

        for start_end in rocks.windows(2) {
            let start = &start_end[0];
            let end = &start_end[1];
            if start.x == end.x {
                let mut y_start = start.y;
                let mut y_end = end.y;
                if y_start > y_end {
                    let tmp = y_start;
                    y_start = y_end;
                    y_end = tmp;
                }
                for y in start.y..=end.y {
                    cave[y][start.x - x_offset] = CaveItem::Rock;
                }
            } else if start.y == end.y {
                let mut x_start = start.x;
                let mut x_end = end.x;
                if x_start > x_end {
                    let tmp = x_start;
                    x_start = x_end;
                    x_end = tmp;
                }
                for x in x_start..=x_end {
                    dbg!(x - x_offset);
                    cave[start.y][x - x_offset] = CaveItem::Rock;
                }
            }
        }
        
        Self { things: cave }
    }

    fn print_cave(&self) {
        for row in self.things.iter() {
            for item in row {
                match item {
                    CaveItem::Sand => print!("o"),
                    CaveItem::Rock => print!("#"),
                    CaveItem::Air => print!("."),
                }
            }
            println!();
        }
    }
}

fn parse_rocks(input: &str) -> Vec<Position<usize>> {
    let mut rocks = vec![];
    for line in input.lines() {
        for item in line.split(" ") {
            if item != "->" {
                let (x, y) = item.split_once(",").unwrap();
                rocks.push(Position::new(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
            }
        }
    }
    rocks 
}

fn part_1(input: &str) -> String {
    let rocks = parse_rocks(input);
    let cave = Cave::new(rocks);
    cave.print_cave();

    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
