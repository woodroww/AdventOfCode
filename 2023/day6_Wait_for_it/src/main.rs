#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_input_1(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace();
    let distances = lines.next().unwrap().split_whitespace();
    times
        .skip(1)
        .zip(distances.skip(1))
        .map(|(time, distance)| Race {
            time: time.parse::<u64>().unwrap(),
            distance: distance.parse::<u64>().unwrap(),
        })
        .collect()
}

fn part_1(input: &str) -> String {
    let races = parse_input_1(input);
    let mut ways_to_win = vec![];
    for race in races {
        println!("Time: {}", race.time);
        println!("Distance: {}", race.distance);
        let mut race_wins = vec![];
        for hold in 1..race.time-1 {
            //let speed = hold;
            let distance = (race.time - hold) * hold;
            if distance > race.distance {
                println!("hold:{}", hold);
                race_wins.push(hold);
            }
        }
        println!("ways to win: {}", race_wins.len());
        ways_to_win.push(race_wins.len());
    }

    for way in &ways_to_win {
        println!("{}", way);
    }
    ways_to_win.into_iter().reduce(|acc, x| acc * x).unwrap().to_string()
}

fn parse_input_2(input: &str) -> Race {
    let mut lines = input.lines();
    let mut times = lines.next().unwrap().split_whitespace();
    let mut distances = lines.next().unwrap().split_whitespace();

    let mut time = String::new();
    times.next();
    for t in times {
        time.push_str(t);
    }
    let mut distance = String::new();
    distances.next();
    for d in distances {
        distance.push_str(d);
    }

    Race {
        time: time.parse::<u64>().unwrap(),
        distance: distance.parse::<u64>().unwrap(),
    }
}

fn part_2(input: &str) -> String {
    let race = parse_input_2(input);
    println!("{:?}", race);
    let mut race_wins = vec![];
    for hold in 1..race.time-1 {
        let distance = (race.time - hold) * hold;
        if distance > race.distance {
            race_wins.push(hold);
        }
    }
    race_wins.len().to_string()
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
        assert_eq!(result, "288");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "2065338");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "71503");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "34934171");
    }
}
