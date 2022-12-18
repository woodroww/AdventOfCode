use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct Point3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub fn parse_cubes(input: &str) -> HashSet<Point3D> {
    let mut cubes: HashSet<Point3D> = HashSet::new();
    for line in input.lines() {
        let mut iter = line.split(",");
        let p = Point3D {
            x: iter.next().unwrap().parse::<usize>().unwrap(),
            y: iter.next().unwrap().parse::<usize>().unwrap(),
            z: iter.next().unwrap().parse::<usize>().unwrap(),
        };
        cubes.insert(p);
    }
    cubes
}

pub fn max_xyz(cubes: &HashSet<Point3D>) -> (usize, usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for cube in cubes.iter() {
        max_x = max_x.max(cube.x);
        max_y = max_y.max(cube.y);
        max_z = max_z.max(cube.z);
    }
    (max_x, max_y, max_z)
}

pub fn min_xyz(cubes: &HashSet<Point3D>) -> (usize, usize, usize) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = 0;

    for cube in cubes.iter() {
        min_x = min_x.min(cube.x);
        min_y = min_y.min(cube.y);
        min_z = min_z.min(cube.z);
    }
    (min_x, min_y, min_z)
}

pub fn center(cubes: &HashSet<Point3D>) -> Point3D {
    let (max_x, max_y, max_z) = max_xyz(&cubes);
    let (min_x, min_y, min_z) = min_xyz(&cubes);
    Point3D {
        x: (max_x + min_x) / 2,
        y: (max_y + min_y) / 2,
        z: (max_z + min_z) / 2, 
    }
}

pub fn part_1(input: &str) -> String {
    let cubes = parse_cubes(input); 
    let (max_x, max_y, max_z) = max_xyz(&cubes);
    surface_area(&cubes, max_x, max_y, max_z).to_string()
}

fn surface_area(cubes: &HashSet<Point3D>, max_x: usize, max_y: usize, max_z: usize) -> usize {
    let mut total = 0;
    for cube in cubes.iter() {
        let result = cardinal_3d(cube, max_x + 2, max_y + 2, max_z + 2);
        let mut surface_area = 6;
        for other_cube in result.iter() {
            if cubes.contains(&other_cube) {
                surface_area -= 1;
            }
        }
        total += surface_area;
    }

   total 
}

pub fn part_2(input: &str) -> String {
    "".to_string()
}

fn cardinal_3d(
    point: &Point3D,
    x_bound: usize,
    y_bound: usize,
    z_bound: usize,
) -> Vec<Point3D> {
    cardinal_directions_3d(point.x, point.y, point.z, x_bound, y_bound, z_bound)
        .iter()
        .map(|(x, y, z)| Point3D { x: *x, y: *y, z: *z })
        .collect()
}

pub fn cardinal_directions_3d(
    x: usize,
    y: usize,
    z: usize,
    x_bound: usize,
    y_bound: usize,
    z_bound: usize,
) -> Vec<(usize, usize, usize)> {
    let mut dirs: Vec<(usize, usize, usize)> = Vec::new();
    let two_d = cardinal_directions(x, y, x_bound, y_bound);
    let positive_z = z.checked_sub(1);
    let negative_z = match z.checked_add(1) {
        Some(z) => {
            if z < z_bound {
                Some(z)
            } else {
                None
            }
        }
        None => None,
    };
    for (x, y) in two_d {
        dirs.push((x, y, z));
    }
    if let Some(z) = positive_z {
        dirs.push((x, y, z));
    }
    if let Some(z) = negative_z {
        dirs.push((x, y, z));
    }
    dirs
}

pub fn cardinal_directions(
    x: usize,
    y: usize,
    x_bound: usize,
    y_bound: usize,
) -> Vec<(usize, usize)> {
    let mut dirs = Vec::new();
    if let Some(x) = x.checked_sub(1) {
        dirs.push((x, y));
    }
    if let Some(y) = y.checked_sub(1) {
        dirs.push((x, y));
    }
    if let Some(x) = x.checked_add(1) {
        if x < x_bound {
            dirs.push((x, y));
        }
    }
    if let Some(y) = y.checked_add(1) {
        if y < y_bound {
            dirs.push((x, y));
        }
    }
    dirs
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
        let result = part_1(&input);
        assert_eq!(result, "64");
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
        assert_eq!(result, "3550");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
