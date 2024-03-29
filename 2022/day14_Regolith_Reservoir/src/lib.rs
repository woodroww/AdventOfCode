#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
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

pub enum CaveItem {
    Sand,
    Rock,
    Air,
}

pub struct Cave {
    pub things: Vec<Vec<CaveItem>>,
    pub x_size: usize,
    pub y_size: usize,
    pub spawner: Position<usize>,
    pub moving_sand: Option<Position<usize>>,
}

pub enum MoveType {
    OffMap,
    Blocked,
    Resting,
    MoveInto(Position<usize>),
}

impl Cave {

    pub fn new(input: &str, part_2: bool) -> Self {
        let rocks = Cave::parse_rocks(input, part_2);
        Cave::cave_from_rocks(rocks)
    }

    fn parse_rocks(input: &str, part_2: bool) -> Vec<Vec<Position<usize>>> {
        let mut rock_lines = vec![];
        for line in input.lines() {
            let mut rocks = vec![];
            for item in line.split(" ") {
                if item != "->" {
                    let (x, y) = item.split_once(",").unwrap();
                    rocks.push(Position::new(
                        x.parse::<usize>().unwrap(),
                        y.parse::<usize>().unwrap(),
                    ));
                }
            }
            rock_lines.push(rocks);
        }

        if part_2 {
            let (max_x, max_y, min_x, min_y) = Cave::max_from_rocks_lines(&rock_lines);
            let mut rocks = vec![];
            rocks.push(Position::new(min_x - 2 * (max_x - min_x), max_y + 2));
            rocks.push(Position::new(max_x + 2 * (max_x - min_x), max_y + 2));
            rock_lines.push(rocks);
        }

        rock_lines 
    }

    fn max_from_rocks_lines(rock_lines: &Vec<Vec<Position<usize>>>) -> (usize, usize, usize, usize) {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        for line in rock_lines.iter() {
            let (line_max_x, line_max_y, line_min_x, line_min_y) = max_xy_min_xy(&line);
            max_x = max_x.max(line_max_x);
            max_y = max_y.max(line_max_y);
            min_x = min_x.min(line_min_x);
            min_y = min_y.min(line_min_y);
        }
        (max_x, max_y, min_x, min_y)
    }

    fn cave_from_rocks(rock_lines: Vec<Vec<Position<usize>>>) -> Self {
        let (max_x, max_y, min_x, min_y) = Cave::max_from_rocks_lines(&rock_lines);

        let x_offset = min_x;
        let sand_spawner = Position::new(500 - x_offset, 0);
        let mut cave = Vec::new();
        for _ in 0..=max_y {
            let mut row: Vec<CaveItem> = Vec::new();
            for _ in 0..=max_x - min_x {
                row.push(CaveItem::Air);
            }
            cave.push(row);
        }

        for rocks in rock_lines {
            for start_end in rocks.windows(2) {
                let start = &start_end[0];
                let end = &start_end[1];
                if start.x == end.x {
                    for y in start.y.min(end.y)..=start.y.max(end.y) {
                        cave[y][start.x - x_offset] = CaveItem::Rock;
                    }
                } else if start.y == end.y {
                    for x in start.x.min(end.x)..=start.x.max(end.x) {
                        cave[start.y][x - x_offset] = CaveItem::Rock;
                    }
                }
            }
        }

        Self {
            things: cave,
            spawner: sand_spawner,
            x_size: max_x - min_x,
            y_size: max_y,
            moving_sand: None,
        }
    }

    fn print_cave(&self) {
        for (y, row) in self.things.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if let Some(mover) = self.moving_sand {
                    if mover == (Position { x, y }) {
                        print!("0");
                    } else {
                        match item {
                            CaveItem::Sand => print!("o"),
                            CaveItem::Rock => print!("#"),
                            CaveItem::Air => print!("."),
                        }
                    }
                } else {
                    match item {
                        CaveItem::Sand => print!("o"),
                        CaveItem::Rock => print!("#"),
                        CaveItem::Air => print!("."),
                    }
                }
            }
            println!();
        }
    }

    fn is_free(&self, pos: &Position<usize>) -> MoveType {
        if pos.y > self.y_size {
            return MoveType::OffMap;
        }
        if pos.x > self.x_size {
            return MoveType::OffMap;
        }
        if let CaveItem::Air = self.things[pos.y][pos.x] {
            MoveType::MoveInto(Position::new(0, 0))
        } else {
            if pos.x == self.spawner.x && pos.y == self.spawner.y {
                MoveType::Blocked
            } else {
                MoveType::Resting
            }
        }
    }

    pub fn can_move_sand_at(&self, current: &Position<usize>) -> MoveType {
        if let CaveItem::Sand = self.things[current.y][current.x] {
            return MoveType::Blocked;
        }
        let down = Position {
            x: current.x,
            y: current.y + 1,
        };
        match self.is_free(&down) {
            MoveType::OffMap => {
                return MoveType::OffMap;
            },
            MoveType::MoveInto(_) => {
                return MoveType::MoveInto(down);
            },
            MoveType::Blocked => {
                return MoveType::Blocked;
            }
            MoveType::Resting => {}
        }

        if current.x > 0 {
            let down_left = Position {
                x: current.x - 1,
                y: current.y + 1,
            };
            match self.is_free(&down_left) {
                MoveType::OffMap => {
                    return MoveType::OffMap;
                },
                MoveType::MoveInto(_) => {
                    return MoveType::MoveInto(down_left);
                },
                MoveType::Resting => {}
                MoveType::Blocked => {
                    return MoveType::Blocked;
                }
            }
        } else {
            return MoveType::OffMap;
        }

        let down_right = Position {
            x: current.x + 1,
            y: current.y + 1,
        };
        match self.is_free(&down_right) {
            MoveType::OffMap => {
                return MoveType::OffMap;
            },
            MoveType::MoveInto(_) => {
                return MoveType::MoveInto(down_right);
            },
            MoveType::Resting => {}
            MoveType::Blocked => {
                return MoveType::Blocked;
            }
        }

        MoveType::Resting
    }

    pub fn step(&mut self) -> MoveType {
        let sand = self.moving_sand.unwrap();
        let result = match self.can_move_sand_at(&sand) {
            MoveType::OffMap => {
                self.moving_sand = None;
                MoveType::OffMap 
            },
            MoveType::MoveInto(pos) => {
                self.moving_sand = Some(pos);
                MoveType::MoveInto(pos)
            },
            MoveType::Blocked => {
                self.things[sand.y][sand.x] = CaveItem::Sand;
                MoveType::Blocked 
            },
            MoveType::Resting => {
                self.things[sand.y][sand.x] = CaveItem::Sand;
                self.moving_sand = None;
                MoveType::Resting
            },
        };
        result
    }

    pub fn spawn_sand_and_step(&mut self) -> bool { // should continue
        let mut continue_loop = true;
        while continue_loop == true {
            if self.moving_sand.is_none() {
                self.moving_sand = Some(self.spawner);
            }
            if self.moving_sand.is_some() {
                match self.step() {
                    MoveType::OffMap => { return false; },
                    MoveType::Blocked => { return false; },
                    MoveType::Resting => { continue_loop = false; },
                    MoveType::MoveInto(new_move) => {
                        self.moving_sand = Some(new_move);
                    }
                }
            }
        }
        true
    }
}


pub fn part_1(input: &str) -> String {
    let mut cave = Cave::new(input, false);
    //cave.print_cave();
    //println!();
    let mut count = 0;
    while cave.spawn_sand_and_step() {
        count += 1;
    }
    //cave.print_cave();
    count.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut cave = Cave::new(input, true);
    let mut count = 0;
    //cave.print_cave();
    println!();
    while cave.spawn_sand_and_step() {
        count += 1;
    }
    //cave.print_cave();
    count.to_string()
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
    fn test_example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "24");
    }

    #[test]
    fn test_example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "93");
    }

    #[test]
    fn test_real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "578");
    }

    #[test]
    fn test_real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "24377");
    }
}
