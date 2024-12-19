use std::collections::{HashMap, HashSet, VecDeque};

struct Rules {
    pages_after_key: HashMap<u32, Vec<u32>>,
    pages_before_key: HashMap<u32, Vec<u32>>,
}

fn parse(input: &str) -> (Rules, Vec<Vec<u32>>) {
    let mut file = input.split("\n\n");
    let rule_txt = file.next().unwrap();
    let update_txt = file.next().unwrap();
    let mut rules = vec![];
    for rule in rule_txt.lines() {
        let mut iter = rule.split('|');
        let before = iter.next().unwrap();
        let after = iter.next().unwrap();
        rules.push((before.parse::<u32>().unwrap(), after.parse::<u32>().unwrap()));
    }
    let mut updates = vec![];
    for up in update_txt.lines() {
        let mut update = vec![];
        let iter = up.split(',');
        for page in iter {
            update.push(page.parse::<u32>().unwrap());
        }
        updates.push(update);
    }
    let mut pages_after_key: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut pages_before_key: HashMap<u32, Vec<u32>> = HashMap::new();
    for (x, y) in rules.iter() {
        pages_before_key.entry(*y).and_modify(|e| e.push(*x)).or_insert(vec![*x]);
        pages_after_key.entry(*x).and_modify(|e| e.push(*y)).or_insert(vec![*y]);
    }
    (Rules { pages_after_key, pages_before_key }, updates)
}

fn update_valid(update: &Vec<u32>, rules: &Rules) -> bool {
    let mut ok = true;
    for (i, x) in update.iter().enumerate() {
        for (j, y) in update.iter().enumerate() {
            if i < j {
                if let Some(rule) = rules.pages_before_key.get(&x) {
                     if rule.contains(y) {
                        ok = false;
                    }
                }
            }
        }
    }
    ok
}

fn reorder_update(update: &Vec<u32>, rules: &Rules) -> Vec<u32> {
    let mut good: Vec<u32> = Vec::new();
    let mut q: VecDeque<u32> = VecDeque::new();
    let update_set: HashSet<u32> = HashSet::from_iter(update.clone());
    let mut big_d: HashMap<u32, usize> = HashMap::new();

    for page in update {
        if let Some(before) = rules.pages_before_key.get(page) {
            let before_set: HashSet<u32> = HashSet::from_iter(before.clone());
            let inter: Vec<&u32> = before_set.intersection(&update_set).collect();
            big_d.entry(*page).and_modify(|e| *e = inter.len()).or_insert(inter.len());
        } else {
            big_d.entry(*page).and_modify(|e| *e = 0).or_insert(0);
        }
    }

    for page in update {
        if *big_d.get(page).unwrap() == 0 {
            q.push_back(*page);
        }
    }

    while let Some(x) = q.pop_front() {
        good.push(x);
        if let Some(erx) = rules.pages_after_key.get(&x) {
            for y in erx {
                if let Some(entry) = big_d.get_mut(y) {
                    *entry -= 1;
                    if *entry == 0 {
                        q.push_back(*y);
                    }
                }
            }
        }
    }
    good
}

fn part_1(input: &str) -> String {
    let (rules, updates) = parse(input);
    let mut result = 0;
    for update in updates {
        if update_valid(&update, &rules) {
            result += update[update.len() / 2];
        }
    }
    result.to_string()
}

fn part_2(input: &str) -> String {
    let (rules, updates) = parse(input);
    let mut result = 0;
    for update in updates {
        if !update_valid(&update, &rules) {
            let new_update = reorder_update(&update, &rules);
            result += new_update[new_update.len() / 2];
        }
    }
    result.to_string()
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
        assert_eq!(result, "143");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "5452");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "123");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "4598");
	}
}
