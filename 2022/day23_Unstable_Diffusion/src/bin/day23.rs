use day23::{input_txt, InputFile, part_1, part_2};

fn main() {
    //let input = input_txt(InputFile::Example);
    //let input = std::fs::read_to_string("small.txt").expect("No example.txt file");
    let input = input_txt(InputFile::Real);

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
