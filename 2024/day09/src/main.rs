use std::collections::{BTreeMap, HashMap, VecDeque};

fn expand(input: &str) -> VecDeque<Option<usize>> {
    let input = input.strip_suffix("\n").unwrap();
    let mut expanded = VecDeque::new();
    let mut id = 0;
    for (i, c) in input.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            let blocks = n;
            for _ in 0..blocks {
                expanded.push_back(Some(id));
            }
            id += 1;
        } else {
            let free_space = n;
            for _ in 0..free_space {
                expanded.push_back(None);
            }
        }
    }
    expanded
}

fn print_block((start, end): (usize, usize), expanded: &VecDeque<Option<usize>>) {
    use colored::Colorize;
    //println!("start: {} end: {} len: {}", start, end, end - start + 1);
    for i in 0..start {
        if let Some(what) = expanded.iter().nth(i).unwrap() {
            print!("{}", what);
        } else {
            print!(".", );
        }
    }
    for i in start..=end {
        if let Some(what) = expanded.iter().nth(i).unwrap() {
            print!("{}", what.to_string().red());
        } else {
            print!("{}", ".".to_string().red());
        }
    }
    for i in end + 1..expanded.len() {
        if let Some(what) = expanded.iter().nth(i).unwrap() {
            print!("{}", what);
        } else {
            print!(".", );
        }
    }
    println!();
}


fn print_expanded<'a, I>(expanded: I)
where
    I: Iterator<Item = &'a Option<usize>>,
{
    println!(
        "{}",
        expanded
            .into_iter()
            .map(|block| {
                if let Some(id) = block {
                    id.to_string()
                } else {
                    ".".to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("")
    );
}

fn part_1(input: &str) -> String {
    let mut expanded = expand(input);
    print_expanded(expanded.iter());
    let mut compacted = Vec::with_capacity(expanded.len());
    while let Some(front) = expanded.pop_front() {
        if let Some(b) = front {
            compacted.push(Some(b));
        } else {
            while let Some(back) = expanded.pop_back() {
                if let Some(b) = back {
                    compacted.push(Some(b));
                    break;
                }
            }
        }
    }

    let mut compacted_string = String::new();
    for block in compacted.iter() {
        if let Some(b) = block {
            compacted_string.push_str(&b.to_string());
        } else {
            compacted_string.push('.');
        }
    }
    println!("{}", compacted_string);

    compacted
        .into_iter()
        .filter_map(|b| b)
        .enumerate()
        .fold(0, |acc, (i, n)| acc + i * n)
        .to_string()
}

fn next_block(blocks: &VecDeque<Option<usize>>, last: &mut usize) -> Option<(usize, (usize, usize))> {
    let mut end = None;
    while end.is_none() {
        match last.checked_sub(1) {
            Some(l) => *last = l,
            None => return None,
        }
        end = *blocks.iter().nth(*last).unwrap();
    }

    let end_index = *last;
    let mut block_id = None;
    let mut prev_block_id = None;
    while let Some(block) = end {
        block_id = Some(block);
        //println!("id: {}, prev: {:?}", block, prev_block_id);
        if let Some(prev) = prev_block_id {
            if block_id == prev {
                match last.checked_sub(1) {
                    Some(l) => *last = l,
                    None => return None,
                }
                end = *blocks.iter().nth(*last).unwrap();
                prev_block_id = Some(block_id);
            } else {
                break;
            }
        } else {
            match last.checked_sub(1) {
                Some(l) => *last = l,
                None => return None,
            }
            end = *blocks.iter().nth(*last).unwrap();
            prev_block_id = Some(block_id);
        }
    }
    *last += 1;
    let start_index = *last;
    Some((prev_block_id.unwrap().unwrap(), (start_index, end_index)))
}

fn calc_empty_space(expanded: &VecDeque<Option<usize>>) -> BTreeMap<usize, usize> {
    let mut empty_spots: BTreeMap<usize, usize> = BTreeMap::new();
    let mut free_start: Option<usize> = None;
    let mut iter = expanded.iter().enumerate();
    while let Some((i, block)) = iter.next()  {
        if let Some(start) = free_start {
            if block.is_none() {
                empty_spots.entry(start).and_modify(|count| *count += 1);
            } else {
                free_start = None;
            }
        } else if block.is_none() {
            free_start = Some(i);
            empty_spots.entry(i).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    println!("{:?}", empty_spots);
    empty_spots
}

fn part_2(input: &str) -> String {
    let mut expanded = expand(input);
    let mut empty_spots = calc_empty_space(&expanded);
    let mut last = expanded.len();
    let mut blocks = vec![];
    while let Some((block_id, (start, end))) = next_block(&expanded, &mut last) {
        blocks.push((block_id, (start, end)));
    }

    for (block_id, (start, end)) in blocks {
        //print_block((start, end), &expanded);
        let length = end - start + 1;
        for (i, empty_len) in empty_spots.iter() {
            if length <= *empty_len {
                for j in *i..*i+length {
                    let what = expanded.iter_mut().nth(j).unwrap();
                    *what = Some(block_id);
                }
                for j in start..=end {
                    let what = expanded.iter_mut().nth(j).unwrap();
                    *what = None;
                }
                let old_len = *empty_spots.get(i).unwrap();
                let key = *i;
                empty_spots.entry(key).and_modify(|v| *v = 0);
                empty_spots.insert(key + length, old_len - length);
                break;
            }
        }
    }

    //println!();
    //print_expanded(expanded.iter());
    //println!("00992111777.44.333....5555.6666.....8888..");

    expanded
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| if b.is_some() { Some((i, b.unwrap())) } else { None })
        .fold(0, |acc, (i, n)| acc + i * n)
        .to_string()
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
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "1928");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "6299243228569");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "2858");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        // 8502539201789 too high
        assert_eq!(result, "0");
    }
}
