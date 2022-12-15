use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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

pub struct Rectangle {
    pub top_left: Position<isize>,
    pub bottom_right: Position<isize>,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "top_left: {}, bottom_right: {}",
            self.top_left, self.bottom_right
        )
    }
}

impl Rectangle {
    pub fn height(&self) -> usize {
        (self.bottom_right.y - self.top_left.y).abs() as usize
    }
    pub fn width(&self) -> usize {
        (self.bottom_right.x - self.top_left.x).abs() as usize
    }
    pub fn x_offset(&self) -> isize {
        self.top_left.x
    }
    pub fn y_offset(&self) -> isize {
        self.top_left.y
    }
    pub fn y_min(&self) -> isize {
        self.top_left.y
    }
    pub fn y_max(&self) -> isize {
        self.bottom_right.y
    }
    pub fn x_min(&self) -> isize {
        self.top_left.x
    }
    pub fn x_max(&self) -> isize {
        self.bottom_right.x
    }
}

#[derive(Debug)]
pub struct Sensor {
    pub location: Position<isize>,
    pub beacon: Position<isize>,
}

impl std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "sensor:{} beacon:{} manhattan:{}",
            self.location,
            self.beacon,
            self.manhattan_distance()
        )
    }
}

fn manhattan_distance(a: &Position<isize>, b: &Position<isize>) -> isize {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

impl Sensor {
    pub fn manhattan_distance(&self) -> isize {
        manhattan_distance(&self.location, &self.beacon)
    }

    pub fn covers_point(&self, point: &Position<isize>) -> bool {
        let sensor = self.manhattan_distance();
        let other = manhattan_distance(&self.location, &point);
        sensor >= other //|| other == 0
    }
}

pub fn parse_input(input: &str) -> Vec<Sensor> {
    let mut result = Vec::new();
    for line in input.lines() {
        for (sensor, beacon) in line.split_once(": ") {
            let (begin, end) = sensor.split_once(", ").unwrap();
            let (_, sensor_x) = begin.rsplit_once(" ").unwrap();
            let (_, sensor_x) = sensor_x.rsplit_once("=").unwrap();
            let (_, sensor_y) = end.rsplit_once("=").unwrap();
            let (begin, end) = beacon.split_once(", ").unwrap();
            let (_, beacon_x) = begin.rsplit_once(" ").unwrap();
            let (_, beacon_x) = beacon_x.rsplit_once("=").unwrap();
            let (_, beacon_y) = end.rsplit_once("=").unwrap();

            result.push(Sensor {
                location: Position {
                    x: sensor_x.parse::<isize>().unwrap(),
                    y: sensor_y.parse::<isize>().unwrap(),
                },
                beacon: Position {
                    x: beacon_x.parse::<isize>().unwrap(),
                    y: beacon_y.parse::<isize>().unwrap(),
                },
            });
        }
    }
    result
}

pub fn set_for_sensor(sensor: &Sensor, grid: &Rectangle) -> HashSet<Position<isize>> {
    let mh = sensor.manhattan_distance();
    let mut count_map = HashSet::new();
    // check if the row we are interested in can be reached within the
    // manhattan distance of the current sensor
    let a = sensor.location.y + mh;
    let b = sensor.location.y - mh;
    //println!("set_for_sensor row_a:{}, row_b:{}", a, b);
    let mut aa = a.min(b);
    let mut bb = a.max(b);
    //println!("set_for_sensor row_a:{}, row_b:{}", aa, bb);
    //println!("grid: {}", grid);

    if aa < grid.y_min() {
        aa = grid.y_min();
    }
    if aa > grid.y_max() {
        aa = grid.y_max();
    }
    if bb < grid.x_min() {
        bb = grid.x_min();
    }
    if bb > grid.x_max() {
        bb = grid.x_max();
    }
    let row_a = aa;
    let row_b = bb;

    for row in row_a..=row_b {
        for y_travel in [-mh, mh] {
            let max_y_travel = sensor.location.y + y_travel;
            let y1 = sensor.location.y.min(max_y_travel);
            let y2 = sensor.location.y.max(max_y_travel);
            if y1 <= row && row <= y2 {
                // if row can be reached
                // calculate how much remaining reach the sensor has after it has moved to the row
                let dist_current = (sensor.location.y - row).abs();
                let dist_remaining = mh - dist_current;
                // then travel that distance left and right and insert those coordinates into map
                let b = sensor.location.x - dist_remaining;
                let e = sensor.location.x + dist_remaining + 1;

                let begin = b.max(grid.x_min());
                let end = e.min(grid.x_max());

                for x in begin..end {
                    count_map.insert(Position { x, y: row });
                }
            }
        }
    }
    count_map
}

// https://github.com/jarshwah/advent-of-code/blob/main/python/2022/q15.py
fn part_1(input: &str, row: isize) -> String {
    let sensors = parse_input(input);
    let mut count_map = HashSet::new();
    for sensor in sensors.iter() {
        let mh = sensor.manhattan_distance();
        // check if the row we are interested in can be reached within the
        // manhattan distance of the current sensor
        for y_travel in [-mh, mh] {
            let max_y_travel = sensor.location.y + y_travel;
            let y1 = sensor.location.y.min(max_y_travel);
            let y2 = sensor.location.y.max(max_y_travel);
            if y1 <= row && row <= y2 {
                // if row can be reached
                // calculate how much remaining reach the sensor has after it has moved to the row
                let dist_current = (sensor.location.y - row).abs();
                let dist_remaining = mh - dist_current;
                // then travel that distance left and right and insert those coordinates into map
                for x in sensor.location.x - dist_remaining..sensor.location.x + dist_remaining + 1
                {
                    count_map.insert(x);
                }
            }
        }
    }
    // minus the beacon
    (count_map.len() - 1).to_string()
}

pub fn max_xy_min_xy(sensors: &Vec<Sensor>) -> (isize, isize, isize, isize) {
    let min_x = sensors
        .iter()
        .map(|s| s.location.x.min(s.beacon.x))
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.location.x.max(s.beacon.x))
        .max()
        .unwrap();
    let min_y = sensors
        .iter()
        .map(|s| s.location.y.min(s.beacon.y))
        .min()
        .unwrap();
    let max_y = sensors
        .iter()
        .map(|s| s.location.y.max(s.beacon.y))
        .max()
        .unwrap();
    (max_x, max_y, min_x, min_y)
}

fn part_1x(input: &str, row: isize) -> String {
    let sensors = parse_input(input);
    let min_x = sensors
        .iter()
        .map(|s| s.location.x.min(s.beacon.x))
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.location.x.max(s.beacon.x))
        .max()
        .unwrap();

    /*println!(
        "min_x:{}, max_x:{}, min_y:{}, max_y:{}",
        min_x, max_x, min_y, max_y
    );*/
    let beacon_map = sensors
        .iter()
        .map(|s| s.beacon)
        .collect::<HashSet<Position<isize>>>();

    let mut count_map = HashSet::new();
    let mut unavailable = 0;
    for x in min_x..=max_x {
        for sensor in sensors.iter() {
            // if sensors.iter().any(|s| s.covers_point(&Position { x, y: row })) {
            if sensor.covers_point(&Position { x, y: row }) {
                if !beacon_map.contains(&Position { x, y: row }) {
                    count_map.insert(Position { x, y: row });
                    unavailable += 1;
                }
            }
        }
    }
    assert_eq!(count_map.len(), unavailable);
    unavailable.to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    //println!("Part 1: {}", part_1b(&input, 10));

    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(&input, 2_000_000));
    //println!("Part 2: {}", part_2(&input));
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
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input, 10);
        assert_eq!(result, "26");
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
        let result = part_1(&input, 2_000_000);
        assert_eq!(result, "5176944");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
