// Well I couldn't get this one, after hourse of going in circles trying recursion and tree like
// datastructures I watched ThePrimeagen complete both parts on twitch very quickly. So this is
// basically his code.
// https://www.twitch.tv/videos/1672855137
// https://github.com/ThePrimeagen/aoc/blob/2022/src/bin/day7.rs

// begin is $ it is a command
// or
// begin is a listing
//      dir dir_name
// or
//      size file_name

fn part_1(input: String) {

    let mut stack = vec![("/", 0)];
    let mut total = 0;
    let mut final_countdown = vec![];

    for line in input.lines() {

        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        let (begin_line, end_line) = line.rsplit_once(" ").unwrap();

        if begin_line.starts_with("$ cd") {
            if end_line == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount <= 100_000 {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_countdown.push((name, amount));
            } else {
                stack.push((end_line, 0));
            }
        } else {
            if let Ok(file_size) = begin_line.parse::<usize>() {
                stack.last_mut().unwrap().1 += file_size;
            } // else dir
        }
    }

    while let Some((dir_name, size)) = stack.pop() {
        final_countdown.push((dir_name, size));
        if let Some((_top_dir, top_size)) = stack.last_mut() {
            *top_size += size;
        }
    }

    println!("part 1: {}", total); // 95437 1449447
    //println!("{:?}", final_count);

    let total_space = 70_000_000;
    let space_to_delete = 30_000_000;

    let free_space = total_space - final_countdown.last().unwrap().1;
    let space_required = space_to_delete - free_space;

    let total = final_countdown
        .into_iter()
        .filter(|(_, amount)| *amount >= space_required)
        .map(|(_, amount)|  amount)
        .min();

    println!("part 2: {}", total.unwrap()); // 24933642 8679207
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
    part_1(input);
    //println!("Part 2: {}", part_2(input));
}

