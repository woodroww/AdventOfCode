use std::{io::BufRead, ops::Range};

fn parse_seeds_1(seeds: &str) -> Vec<usize> {
    let splitsies = seeds.split(": ");
    let seeds_to_plant: Vec<usize> = splitsies
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    seeds_to_plant
}

fn parse_seeds_2(seeds: &str) -> Vec<usize> {
    let splitsies = seeds.split(": ");
    let seed_ranges: Vec<usize> = splitsies
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!("seed ranges: {:?}", seed_ranges);
    let mut seeds = vec![];
    for range in seed_ranges.chunks(2) {
        let start = range[0];
        let len = range[1];
        let seed_numbers: Vec<usize> = (start..start+len).collect();
        seeds.extend_from_slice(&seed_numbers);
    }
    seeds
}

struct RangeMap {
    source: Range<usize>,
    destination: Range<usize>,
}

impl RangeMap {
    fn src_to_dst(&self, src: usize) -> Option<usize> {
        if self.source.contains(&src) {
            /*
            println!("dest:{} src:{} len:{}",
                self.destination.start,
                self.source.start,
                self.source.end - self.source.start);
            */
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
    let dest_string = src_dest.next().unwrap();
    let mut destination_string = dest_string.split(" ");
    let destination_string = destination_string.next().unwrap();
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

fn parse_input(input: &str, part: u8) -> (Vec<usize>, Vec<AlmanacMap>) {
    let mut sections = input.split("\n\n");
    let seeds = if part == 1 {
        parse_seeds_1(sections.next().unwrap())
    } else if part == 2 {
        parse_seeds_2(sections.next().unwrap())
    } else {
        panic!("part must be 1 or 2");
    };
    let mut maps = vec![];
    for section in sections.into_iter() {
        maps.push(parse_map(section.trim_end()));
    }
    (seeds, maps)
}

fn part_1(input: &str) -> String {
    let (seeds, maps) = parse_input(input, 1);
    /*
    println!("seeds: {:?}", seeds);
    for map in &maps {
        println!("{}", map);
    }
    */
    convert_seeds(seeds, maps)
}

fn convert_seeds(seeds: Vec<usize>, maps: Vec<AlmanacMap>) -> String {
    let mut min_location = usize::MAX;
    for seed in &seeds {
        println!("seed:{}", seed);
        let mut prev_value = None;
        for map in &maps {
            if prev_value.is_none() {
                prev_value = Some(*seed);
            };
            let value = map.src_to_dst(prev_value.unwrap());
            //println!("map: {}-to-{}", map.source_string, map.destination_string);
            println!("{}:{}", map.destination_string, value);
            prev_value = Some(value);
        }
        let location = prev_value.unwrap();
        if location < min_location {
            min_location = location;
        }
        println!();
    }

    min_location.to_string()
}

fn part_2(input: &str) -> String {
    let (seeds, maps) = parse_input(input, 2);
    convert_seeds(seeds, maps)
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
        let (_seeds, maps) = parse_input(&input, 1);
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
        assert_eq!(result, "579439039");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "46");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
