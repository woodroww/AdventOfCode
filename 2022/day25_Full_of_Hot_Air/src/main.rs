fn snafu_to_decimal(line: &str) -> i64 {
    let mut snafu: Vec<i8> = Vec::new();
    for c in line.chars().rev() {
        if c == '=' {
            snafu.push(-2);
        } else if c == '-' {
            snafu.push(-1);
        } else {
            snafu.push(c.to_digit(10).unwrap().try_into().unwrap());
        }
    }
    // [3125, 625, 125,  25,   5,   1]
    // [ 5^5, 5^4, 5^3, 5^2, 5^1, 5^0]
    // the snafu Vec is reversed, (5^0 at index 0, 5^1 at index 1, ...)
    let mut decimal: i64 = 0;
    for (exponent, num) in snafu.into_iter().enumerate() {
        let fives = i64::pow(5, exponent as u32);
        decimal += fives * num as i64;
    }
    decimal
}

/*

1=-0-2
[1, -2, -1, 0, -1, 2]
[ 5^5, 5^4, 5^3, 5^2, 5^1, 5^0]
[3125, 625, 125,  25,   5,   1]
[3125*1, 625*-2, 125*-1, 25*0, 5*-1, 1*2]
3125 - 1250 - 125 + 0 - 5 + 2
1747

1747
['2', '4', '4', '3', '2']
[3125, 625*2, 125*3,  25*4,   5*4,   1*2]
                             
1=-0-2

n = number in base 10 = 4
base = the base to convert to = 2
4 / base = 2 R 0
2 / base = 1 R 0
1 / base = 0 R 1 stop when quotient is 0

the number 4 in base 2 is 100
[2^2, 2^1, 2^0]
[1, 0, 0]
[4*1, 2*0, 1*0]

    // valid SNAFU digits [2, 1, 0, -, =]
    // values             [2, 1, 0, -1, -2]
*/

    // thanks to 
    // https://www.reddit.com/r/adventofcode/comments/zur1an/comment/j1o2rxj/?utm_source=share&utm_medium=web2x&context=3
    // https://github.com/EricKalkman/AoC2022/blob/master/day25.rkt
    // for the subtract 5 idea

fn decimal_to_snafu(mut decimal: i64) -> String {
    let base = 5;
    let mut keep_going = true;
    let mut base_five_vec = Vec::new();
    while keep_going {
        let quotient = decimal / base;
        let remainder = decimal % base;
        decimal = quotient;
        base_five_vec.push(remainder);
        if quotient == 0 {
            keep_going = false;
        }
    }

    // snafu_vec should be [1, -2, -1, 0, -1, 2]
    // base_five_vec is    [2, 4, 4, 3, 2]

    let mut snafu_vec: Vec<i64> = Vec::new();
    let mut carried = false;
    for n in base_five_vec.into_iter() {
        let mut number = n;
        if carried == true {
            number += 1;
            carried = false;
        }
        if number > 2 {
            snafu_vec.push(number - 5);
            carried = true;
        } else {
            snafu_vec.push(number);
        }
    }
    if carried == true {
        snafu_vec.push(1);
    }

    let snafu = snafu_vec.into_iter().rev().collect::<Vec<i64>>();
    //println!("snafu_vec should be [1, -2, -1, 0, -1, 2]");
    //println!("snafu_vec           {:?}", snafu);
    let mut result_string = String::new();
    for n in snafu.into_iter() {
        let c = match n {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!("what is this?")
        };
        result_string.push(c);
    }

    result_string 
}

fn part_1(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let decimal = snafu_to_decimal(line);
        total += decimal;
        //let back_to_snafu = decimal_to_snafu(decimal);
        //assert_eq!(line, back_to_snafu);
    }
    let result = decimal_to_snafu(total);
    result.to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = input_txt(InputFile::Real);
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
        InputFile::Example => {
            std::fs::read_to_string("example.txt")
                .expect("No example.txt file")
        },
        InputFile::Real => {
            std::fs::read_to_string("input.txt")
                .expect("No input.txt file")
        },
    }
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
		let result = part_1(&input);
        assert_eq!(result, "2=-1=0");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "2=000=22-0-102=-1001");
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
