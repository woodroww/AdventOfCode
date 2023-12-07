use std::{io::BufRead, ops::Range};

fn parse_seeds(seeds: &str) -> Vec<u8> {
    let splitsies = seeds.split(": ");
    let seeds_to_plant: Vec<u8> = splitsies
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    seeds_to_plant
}

struct RangeMap {
    source: Range<usize>,
    destination: Range<usize>,
}

impl RangeMap {
    fn src_to_dst(&self, src: usize) -> Option<usize> {
        if self.source.contains(&src) {
            println!("dest:{} src:{} len:{}",
                self.destination.start,
                self.source.start,
                self.source.end - self.source.start);
            Some((src - self.source.start) + self.destination.start)
        } else {
            None
        }
    }
}

struct AlmanacMap {
    destination_string: String,
    source_string: String,
    ranges: Vec<RangeMap>,
}

impl AlmanacMap {
    fn src_to_dst(&self, src: usize) -> usize {
        for range in &self.ranges {
            if let Some(dest) = range.src_to_dst(src) {
                return dest
            }
        }
        src
    }
}

impl std::fmt::Display for AlmanacMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for range in &self.ranges {
            write!(
                f,
                "{}-to-{} {} {} {}\n",
                self.source_string,
                self.destination_string,
                range.destination.start,
                range.source.start,
                range.source.end - range.source.start
            )?
        }
        write!(f, "")
    }
}

fn parse_map(input: &str) -> AlmanacMap {
    let mut splitsies = input.split("\n");
    let mut ranges = vec![];
    let title = splitsies.next().unwrap();
    let mut src_dest = title.split("-to-");
    let source_string = src_dest.next().unwrap();
    let destination_string = src_dest.next().unwrap();
    for line in splitsies {
        let mut map_parts = line.split(" ");
        let dest = map_parts.next().unwrap().parse::<usize>().unwrap();
        let src = map_parts.next().unwrap().parse::<usize>().unwrap();
        let length = map_parts.next().unwrap().parse::<usize>().unwrap();
        ranges.push(RangeMap {
            source: Range { start: src, end: src + length },
            destination: Range { start: dest, end: dest + length }, 
        });
    }
    AlmanacMap {
        destination_string: destination_string.to_string(),
        source_string: source_string.to_string(),
        ranges
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<AlmanacMap>) {
    let mut sections = input.split("\n\n");
    let seeds = parse_seeds(sections.next().unwrap());
    let mut maps = vec![];
    for section in sections.into_iter() {
        maps.push(parse_map(section.trim_end()));
    }
    (seeds, maps)
}

fn part_1(input: &str) -> String {
    let (seeds, maps) = parse_input(input);
    println!("seeds: {:?}", seeds);
    for map in &maps {
        println!("{}", map);
    }

    let seed = 13;
    println!("seed:{} soil:{}", seed, maps[0].src_to_dst(seed));


    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    /*
    let src = 50;
    let range_length = 2;
    let range = src..src+range_length;
    println!("{:?}", range);
    for r in range {
        print!("{} ", r);
    }
    println!();
    */
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
        InputFile::Example => std::fs::read_to_string("example.txt").expect("No example.txt file"),
        InputFile::Real => std::fs::read_to_string("input.txt").expect("No input.txt file"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_to_soil() {
        let seeds_expected = vec![79, 14, 55, 13];
        let soils_expected = vec![81, 14, 57, 13];
        let input = input_txt(InputFile::Example);
        let (_seeds, maps) = parse_input(&input);
        for (seed, soil) in seeds_expected.into_iter().zip(soils_expected) {
            let soil_result = maps[0].src_to_dst(seed);
            assert_eq!(soil, soil_result);
        }
    }

    #[test]
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "35");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
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
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
