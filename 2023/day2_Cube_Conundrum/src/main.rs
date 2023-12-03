pub enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct Game {
    rounds: Vec<Round>,
    id: u8,
}

fn parse_records(input: &str) -> Vec<Game> {
    let mut game_list = vec![];
    for line in input.lines() {
        //println!("text: {}", line);
        let mut split_iter = line.split(":");
        let what = split_iter.next().unwrap();
        let mut split2_iter = what.rsplit(" ");
        let game_id = split2_iter.next().unwrap();
        //println!("game_id: {}", game_id);
        let games = split_iter.next().unwrap();
        let rounds = games.split(";");
        let mut round_list = vec![];
        for round in rounds {
            //println!("round:{}", round);
            let mut round_data = Round { red: 0, green: 0, blue: 0 };
            for cube in round.split(",") {
                //println!("\tcube: {}", cube);
                let cube_trimmed = cube.trim();
                let mut cube_info = cube_trimmed.split(" ");
                let count = cube_info.next().unwrap(); 
                //println!("\tcount: {}", count);
                let color = cube_info.next().unwrap(); 
                //println!("\tcolor: {}", color);
                match color {
                    "red" => round_data.red = count.parse::<u8>().unwrap(),
                    "green" => round_data.green = count.parse::<u8>().unwrap(),
                    "blue" => round_data.blue = count.parse::<u8>().unwrap(),
                    _ => panic!(),
                }
            }
            round_list.push(round_data);
        }

        game_list.push(Game { id: game_id.parse::<u8>().unwrap(), rounds: round_list });
    }
    //println!("{:#?}", game_list);
    game_list
}

fn round_possible(round: Round, bag: &Round) -> bool {
    round.red <= bag.red && round.green <= bag.green && round.blue <= bag.blue
}

fn part_1(input: &str) -> String {
    let game_list = parse_records(input);
    let bag = Round { red: 12, green: 13, blue: 14 };
    let mut game_id_total: u16 = 0;

    for game in game_list {
        let mut possible = true;
        for round in game.rounds {
            if !round_possible(round, &bag) {
                possible = false;
            }
        }
        if possible {
            game_id_total += game.id as u16;
        }
    }

    game_id_total.to_string()
}

fn part_2(input: &str) -> String {
    let game_list = parse_records(input);
    let mut max_games = vec![];
    for game in game_list {
        let mut max_round = Round { red: 0, green: 0, blue: 0 };
        for round in game.rounds {
            if round.red > max_round.red {
                max_round.red = round.red;
            }
            if round.green > max_round.green {
                max_round.green = round.green;
            }
            if round.blue > max_round.blue {
                max_round.blue = round.blue;
            }
        }
        max_games.push(max_round);
    }
    
    let mut sum = 0;
    for game in max_games {
        let power: u32 = game.red as u32 * game.green as u32 * game.blue as u32;
        //println!("{}", power);
        sum += power;
    }
    sum.to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

    //println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

pub enum InputFile {
    Example,
    Real,
}

pub fn input_txt(input: InputFile) -> String {
    match input {
        InputFile::Example => std::fs::read_to_string("example.txt").expect("No example.txt file"),
        InputFile::Real => std::fs::read_to_string("input.txt").expect("No input.txt file"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part_1() {
        let input = input_txt(InputFile::Example);
        let result = part_1(&input);
        assert_eq!(result, "8");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "2600");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "2286");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "86036");
    }
}
