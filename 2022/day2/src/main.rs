fn input_txt(use_example: bool) -> String {
    if use_example {
        std::fs::read_to_string("example.txt")
            .expect("No example.txt file")
    } else {
        std::fs::read_to_string("input.txt")
            .expect("No input.txt file")
    }
}

// A rock
// B paper
// C scissors
// X rock       1
// Y paper      2
// Z scissors   3
// win  6
// draw 3
// loss 0

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("what?"),
        }
    }
}


#[derive(Debug, Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Play {
    fn from(c: char) -> Self {
        match c {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            'X' => Play::Rock,
            'Y' => Play::Paper,
            'Z' => Play::Scissors,
            _ => panic!("what?"),
        }
    }
}

fn main() {
    let input = input_txt(false);
    //part_1(input);
    part_2(input);
}

fn part_1(input: String) {
    let mut results: Vec<u16> = Vec::new();
    for round in input.lines() {
        let mut s = round.split(" ");
        let opponent = s.next().unwrap().chars().nth(0).unwrap();
        let me = s.next().unwrap().chars().nth(0).unwrap();
        let outcome = determine_outcome(&opponent.into(), &me.into());
        let score = determine_score(&outcome, &me.into());
        results.push(score);
    }
    let total: u16 = results.iter().sum();
    println!("{}", total);
}

fn part_2(input: String) {
    let mut results: Vec<u16> = Vec::new();
    for round in input.lines() {
        let mut s = round.split(" ");
        let opponent = s.next().unwrap().chars().nth(0).unwrap();
        let desired_outcome = s.next().unwrap().chars().nth(0).unwrap();
        let choice = determine_choice(&desired_outcome.into(), &opponent.into());
        let score = determine_score(&desired_outcome.into(), &choice);
        results.push(score);
    }
    let total: u16 = results.iter().sum();
    println!("{}", total);
}

fn determine_score(outcome: &Outcome, me: &Play) -> u16 {
    let mut score = match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    };
    score += match me {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };
    score
}

fn determine_choice(desired_outcome: &Outcome, opponent: &Play) -> Play {
    match desired_outcome {
        Outcome::Win => {
            match opponent {
                Play::Rock => Play::Paper,
                Play::Paper => Play::Scissors,
                Play::Scissors => Play::Rock,
            }
        },
        Outcome::Lose => {
            match opponent {
                Play::Rock => Play::Scissors,
                Play::Paper => Play::Rock,
                Play::Scissors => Play::Paper,
            }
        },
        Outcome::Draw => {
            opponent.clone()
        }
    }
}

fn determine_outcome(opponent: &Play, me: &Play) -> Outcome {
    match opponent {
        Play::Rock => {
            match me {
                Play::Rock => Outcome::Draw,
                Play::Paper => Outcome::Win,
                Play::Scissors => Outcome::Lose,
            }
        },
        Play::Paper => {
            match me {
                Play::Rock => Outcome::Lose,
                Play::Paper => Outcome::Draw,
                Play::Scissors => Outcome::Win,
            }
        },
        Play::Scissors => {
            match me {
                Play::Rock => Outcome::Win,
                Play::Paper => Outcome::Lose,
                Play::Scissors => Outcome::Draw,
            }
        },
    }
}
