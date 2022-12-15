use day15::{part_1, part_2, input_txt, InputFile};

fn main() {
    //let input = input_txt(InputFile::Example);
    //println!("Part 1: {}", part_1b(&input, 10));

    let input = input_txt(InputFile::Real);
    println!("Part 1: {}", part_1(&input, 2_000_000));
    //println!("Part 2: {}", part_2(&input));
}

