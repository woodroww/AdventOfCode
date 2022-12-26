use std::collections::HashMap;

struct DirListing {
    name: String,
    sub_dirs: Vec<String>,
    files: Vec<FileListing>,
    size: usize,
}

impl DirListing {
    fn new(name: String) -> Self {
        Self {
            name,
            sub_dirs: Vec::new(),
            files: Vec::new(),
            size: 0,
        }
    }
}

struct FileListing {
    name: String,
    size: usize,
}

// begin is $ it is a command
// or
// begin is a listing
//      dir dir_name
// or
//      size file_name

fn part_1(input: String) -> usize {
    let mut dirs: HashMap<String, DirListing> = HashMap::new();
    dirs.insert("/".to_string(), DirListing::new("/".to_string()));

    for line in input.lines() {
        let (begin, end) = line.rsplit_once(' ').unwrap();
        if begin.starts_with("dir") {
            let dir_listing = DirListing::new(end.to_string());
            let old_value = dirs.insert(end.to_string(), dir_listing);
            if old_value.is_some() {
                println!("we already have this dir in the hashmap");
                panic!();
            }
        }
    }

    let mut line_iter = input.lines();
    let mut current_dir = "".to_string();
    while let Some(line) = line_iter.next() {
        let (begin, end) = line.rsplit_once(' ').unwrap();
        if begin.starts_with("$ cd") {
            current_dir = end.to_string();
        } else if begin.starts_with("dir") {
            dirs.entry(current_dir.clone()).and_modify(|entry| {
                entry.sub_dirs.push(end.to_string());
            });
        } else if begin.chars().nth(0).unwrap().is_numeric() {
            dirs.entry(current_dir.clone()).and_modify(|entry| {
                entry.files.push(FileListing {
                    name: end.to_string(),
                    size: begin.parse::<usize>().unwrap(),
                });
            });
        }
    }

    let root = dirs.get("/").unwrap();
    //print_listing(&dirs, root, 0);

    let mut results: HashMap<String, usize> = HashMap::new();
    dir_size(&dirs, root, &mut results);

    let mut total: usize = 0;
    let at_most: usize = 100_000;
    for (_dir_name, dir_size) in results {
        //println!("{} : {}", dir_name, dir_size);
        if dir_size <= at_most {
            total += dir_size;
        }
    }

    total
}

fn dir_size(
    dirs: &HashMap<String, DirListing>,
    listing: &DirListing,
    mut results: &mut HashMap<String, usize>,
) -> usize {
    let mut size: usize = 0;
    for dir in listing.sub_dirs.iter() {
        let sub_listing = dirs.get(dir).unwrap();
        size += dir_size(dirs, sub_listing, &mut results);
    }
    size = listing.files.iter().fold(size, |acc, file| acc + file.size);
    results.insert(listing.name.clone(), size);

    //println!("{}:{}", listing.name, size);
    size
}

fn print_listing(dirs: &HashMap<String, DirListing>, listing: &DirListing, level: usize) {
    for dir in listing.sub_dirs.iter() {
        for _ in 0..level {
            print!(" ");
        }
        println!("- {} (dir)", dir);
        let dir_listing = dirs.get(dir).unwrap();
        print_listing(dirs, dir_listing, level + 4);
    }
    for file in listing.files.iter() {
        for _ in 0..level {
            print!(" ");
        }
        println!("- {} (file, size={})", file.name, file.size);
    }
}

fn part_2(input: String) -> i32 {
    0
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
    println!("Part 1: {}", part_1(input.clone()));
    //println!("Part 2: {}", part_2(input));
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
