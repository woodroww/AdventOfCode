/*
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from
here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with
our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been
allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according
to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
Each line gives the password policy and then the password. The password policy indicates the lowest
and highest number of times a given letter must appear for the password to be valid. For example,
1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no
instances of b, but needs at least 1. The first and third passwords are valid: they contain one a
or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?

Your puzzle answer was 572.

--- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official
Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from
his old job at the sled rental place down the street! The Official Toboggan Corporate Policy
actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2
means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of
"index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of
the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
*/

use std::fs;
use std::str::FromStr;

struct Password {
    pass: String,
    letter: char,
    min: usize,
    max: usize,
}

struct ParsePasswordError;

impl Password {

    fn is_valid(&self) -> bool {
 
        let mut found_count = 0;

        if self.min < self.pass.len() {
            if self.letter == self.pass.chars().nth(self.min - 1).unwrap() {
                found_count += 1;
            }
        }
        if self.max < self.pass.len() {
            if self.letter == self.pass.chars().nth(self.max - 1).unwrap() {
                found_count += 1;
            }
        }
        if found_count == 1 {
            return true;
        }
        false
    }
}

// 1-3 a: abcde
impl FromStr for Password {

    type Err = ParsePasswordError;

    fn from_str(password_str: &str) -> Result<Self, Self::Err> {
        let splitsies: Vec<&str> = password_str.split(' ').collect();
        if splitsies.len() != 3 {
            return Err(ParsePasswordError);
        }
        
        let min_max: Vec<&str> = splitsies[0].split('-').collect();
        if min_max.len() != 2 {
            return Err(ParsePasswordError);
        }
       
        Ok(Self {
            pass: splitsies[2].to_string(),
            letter: splitsies[1].chars().nth(0).unwrap(),
            min: min_max[0].parse::<usize>().unwrap(),
            max: min_max[1].parse::<usize>().unwrap(),
        })
    }
}

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Couldn't read file");
    let lines: Vec<&str> = contents.split('\n').collect();
    
    let mut passwords: Vec<Password> = Vec::new();

    for line in lines {
        let pass = Password::from_str(line).ok();
        if let Some(p) = pass {
            passwords.push(p);
        }
    }
    let mut count = 0;
    for p in passwords {
        if p.is_valid() {
            count += 1;
        }
    }
    println!("{} valid passwords.", count);
}





