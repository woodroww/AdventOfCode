use std::fs;

fn input_txt(use_example: bool) -> String {
    if use_example {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
        .to_string()
    } else {
        fs::read_to_string("input.txt")
            .expect("No input.txt file")
            .parse()
            .expect("Couldn't parse file")
    }
}

fn main() {
    let mut text = input_txt(false);
    text.pop();

    let mut elf_totals = Vec::new();
    for elf in text.split("\n\n") {
        elf_totals.push(elf.split("\n").fold(0, |acc, item| {
            acc + item.parse::<u32>().expect("couldn't parse line into u32")
        }));
    }

    // part 1
    println!("max calories {}", elf_totals.iter().max().unwrap());

    // part 2
    elf_totals.sort();
    let top_three_total: u32 = elf_totals.iter().rev().take(3).sum();
    println!("top_three_total {}", top_three_total);
}
