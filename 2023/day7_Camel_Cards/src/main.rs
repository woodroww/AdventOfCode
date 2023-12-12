use std::collections::HashMap;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Card {
    Card2,
    Card3,
    Card4,
    Card5,
    Card6,
    Card7,
    Card8,
    Card9,
    CardT,
    CardJ,
    CardQ,
    CardK,
    CardA
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Card2,
            '3' => Card::Card3,
            '4' => Card::Card4,
            '5' => Card::Card5,
            '6' => Card::Card6,
            '7' => Card::Card7,
            '8' => Card::Card8,
            '9' => Card::Card9,
            'T' => Card::CardT,
            'J' => Card::CardJ,
            'Q' => Card::CardQ,
            'K' => Card::CardK,
            'A' => Card::CardA,
            _ => panic!("unexpected input")
        }
    }
}

impl From<&Card> for char {
    fn from(value: &Card) -> Self {
        match value {
            Card::Card2 => '2',
            Card::Card3 => '3',
            Card::Card4 => '4',
            Card::Card5 => '5',
            Card::Card6 => '6',
            Card::Card7 => '7',
            Card::Card8 => '8',
            Card::Card9 => '9',
            Card::CardT => 'T',
            Card::CardJ => 'J',
            Card::CardQ => 'Q',
            Card::CardK => 'K',
            Card::CardA => 'A',
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<char>::into(self))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
enum HandStrength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl From<HandStrength> for &str {
    fn from(value: HandStrength) -> Self {
        match value {
            HandStrength::HighCard => "High card",
            HandStrength::OnePair => "One pair",
            HandStrength::TwoPair => "Two pair",
            HandStrength::ThreeKind => "Three of a kind",
            HandStrength::FullHouse => "Full house",
            HandStrength::FourKind => "Four of a kind",
            HandStrength::FiveKind => "Five of a kind",
        }
    }
}

impl std::fmt::Display for HandStrength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u16,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let strength = self.hand_strength();
        let other_strength = other.hand_strength();
        if strength == other_strength {
            for (card, other_card) in self.cards.iter().zip(&other.cards) {
                if card != other_card {
                    if card > other_card {
                        return std::cmp::Ordering::Greater;
                    } else {
                        return std::cmp::Ordering::Less;
                    }
                }
            }
            std::cmp::Ordering::Equal
        } else {
            if strength > other_strength {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        }
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_strength() == other.hand_strength()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.cards {
            write!(f, "{}", card)?;
        }
        write!(f, " {}", self.bid)
    }
}

impl Hand {
    fn hand_strength(&self) -> HandStrength {
        let mut hand_map = HashMap::new();
        for card in &self.cards {
            let entry = hand_map.entry(card).or_insert(0);
            *entry += 1;
        }
        if hand_map.len() == 1 {
            HandStrength::FiveKind
        } else if hand_map.len() == 2 {
            for (_key, value) in &hand_map {
                if *value == 4 {
                    return HandStrength::FourKind;
                }
            }
            HandStrength::FullHouse
        } else if hand_map.len() == 3 {
            for (_key, value) in &hand_map {
                if *value == 3 {
                    return HandStrength::ThreeKind;
                }
            }
            HandStrength::TwoPair
        } else if hand_map.len() == 4 {
            HandStrength::OnePair
        } else if hand_map.len() == 5 {
            HandStrength::HighCard
        } else {
            panic!("what hand is this")
        }
    }
}

fn parse_hands(input: &str) -> Vec<Hand> {
    let mut hands = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let hand = split.next().unwrap();
        let bid = split.next().unwrap().parse::<u16>().unwrap(); 
        let mut cards: Vec<Card> = vec![];
        for c in hand.chars() {
            cards.push(c.into());
        }
        hands.push(Hand { cards, bid })
    }
    hands
}

fn part_1(input: &str) -> String {
    let mut hands = parse_hands(input);
    hands.sort();
    let mut sum = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        //print!("{}", h);
        //println!(" {}", h.hand_strength());
        let winnings = hand.bid as usize * (i + 1);
        sum += winnings;
    }
    sum.to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
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
        assert_eq!(result, "6440");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "250946742");
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
