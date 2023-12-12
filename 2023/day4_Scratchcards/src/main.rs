use std::{collections::{HashSet, VecDeque, BinaryHeap}, cmp::Ordering};

#[derive(Clone, Eq, PartialEq)]
struct ScratchCard {
    card_n: usize,
    winners: Vec<u8>,
    numbers: Vec<u8>,
}

impl Ord for ScratchCard {
    fn cmp(&self, other: &Self) -> Ordering {
        other.card_n.cmp(&self.card_n)
    }
}

impl PartialOrd for ScratchCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for ScratchCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Card: {} {:?} | {:?}", self.card_n, self.winners, self.numbers)
    }
}

fn parse_one_card(i: usize, line: &str) -> ScratchCard {
    let mut winners = vec![];
    let mut numbers = vec![];
    let mut splitsies = line.split("|");
    let left = splitsies.next().unwrap();
    let right = splitsies.next().unwrap();

    let mut left_splitsies = left.split(": ");
    left_splitsies.next();
    let winning_strings = left_splitsies.next().unwrap();
    let winning_strings = winning_strings.trim();
    for winner in winning_strings.split_whitespace() {
        winners.push(winner.parse::<u8>().unwrap());
    }
    
    let right = right.split_whitespace();
    for possible in right {
        numbers.push(possible.parse::<u8>().unwrap());
    }
    ScratchCard {
        card_n: i + 1,
        winners,
        numbers,
    }
}

fn parse_cards(input: &str) -> Vec<ScratchCard> {
    let mut result = vec![];
    for (i, line) in input.lines().enumerate() {
        let card = parse_one_card(i, line);
        result.push(card);
    }
    result
}

fn part_1(input: &str) -> String {
    let cards = parse_cards(input);
    let mut sum = 0;
    for card in cards {
        let winner: HashSet<u8> = card.winners.into_iter().collect();
        let possible: HashSet<u8> = card.numbers.into_iter().collect();
        let inter: Vec<u8> = winner.intersection(&possible).map(|a| *a).collect();
        let mut points = 0;
        if inter.len() >= 1 {
            points = 1;
        }
        for _ in 1..inter.len() {
            points *= 2;
        }
        sum += points;
    }
    sum.to_string()
}

fn process_card(card: &ScratchCard, original_cards: &Vec<ScratchCard>) -> Vec<ScratchCard> {
    let mut won_cards = Vec::new();
    let card = &original_cards[card.card_n - 1];
    println!("process_card {}", card);
    let winner: HashSet<u8> = card.winners.iter().copied().collect();
    let possible: HashSet<u8> = card.numbers.iter().copied().collect();
    let inter: Vec<u8> = winner.intersection(&possible).map(|a| *a).collect();
    won_cards.extend(original_cards.iter().skip(card.card_n).take(inter.len()).cloned());
    won_cards
}

fn part_2(input: &str) -> String {
    let original_cards = parse_cards(input);
    let mut cards: VecDeque<ScratchCard> = original_cards.clone().into_iter().collect();
    let mut i: usize = 0;
    while cards.len() > 0 {
        let won_cards = process_card(cards.iter().nth(i).unwrap(), &original_cards);
        i += 1;
        if won_cards.len() == 0 {
            break;
        } else {
            cards.extend(won_cards);
        }
        for card in &cards {
            println!("{}", card);
        }
    }
    cards.len().to_string()
}

fn main() {
    let input = input_txt(InputFile::Example);
    //let input = input_txt(InputFile::Real);

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
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
        assert_eq!(result, "13");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "23028");
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
