
use day18::{input_txt, InputFile, part_1, part_2};

fn main() {
    let input = input_txt(InputFile::Example);
    //let input = input_txt(InputFile::Real);
// 2411 3538 3545 all too low
    // 3550 good wasn't counting the surfaces beyond zero
    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
