use std::collections::{HashMap, HashSet};

impl std::fmt::Display for ValveLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.valve_map)?;
        write!(f, "{:#?}", self.flows)
    }
}

fn parse_input(input: &str) -> ValveLayout {
    let mut valve_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut flows = HashMap::new();

    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        //println!("{}", parts[1]);
        let valve = parts[1];
        let flow = parts[4].strip_suffix(';').unwrap();
        let (_, flow) = flow.rsplit_once('=').unwrap();
        let flow = flow.parse::<usize>().unwrap();
        let already_existed = flows.insert(valve.to_string(), flow);
        assert!(already_existed == None);
        //println!("{}", flow);
        let valve_entry = valve_map.entry(valve.to_string()).or_insert(vec![]);
        for part in &parts[9..] {
            let leads_to = match part.strip_suffix(',') {
                Some(s) => s,
                None => part,
            };
            //println!("{}", leads_to);
            valve_entry.push(leads_to.to_string());
        }
        //println!();
    }

    ValveLayout { valve_map, flows }
}

// https://dreampuf.github.io/GraphvizOnline

fn graph_viz_string(layout: &ValveLayout) -> String {
    let mut vis_ids: HashMap<String, usize> = HashMap::new();
    let mut vis_string = String::new();
    for (i, (valve, _leads_to)) in layout.valve_map.iter().enumerate() {
        vis_ids.insert(valve.to_string(), i);
        vis_string.push_str(&format!("{} [ label = \"{}\" ]\n", i, valve));
    }
    for (valve, leads_to) in layout.valve_map.iter() {
        let valve_id = vis_ids.get(valve).unwrap();
        for lead in leads_to.iter() {
            let lead_id = vis_ids.get(lead).unwrap();
            vis_string.push_str(&format!("{} -- {} [ ]\n", valve_id, lead_id));
        }
    }
    vis_string
}

enum CaveAction {
    OpenValve(String),
    Move(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FancyKey {
    current: String,
    time: i64,
    other_players: i64,
    opened_valves: Vec<String>,
}

struct ValveLayout {
    valve_map: HashMap<String, Vec<String>>,
    flows: HashMap<String, usize>,
}

impl ValveLayout {
    fn maxflow(
        &self,
        current: String,
        time: i64,
        other_players: i64,
        memo: &mut HashMap<FancyKey, i64>,
        mut opened_valves: Vec<String>,
    ) -> i64 {
        if time == 0 {
            return if other_players > 0 {
                self.maxflow("AA".to_string(), 26, other_players - 1, memo, opened_valves)
            } else {
                0
            };
        }

        let key = FancyKey {
            current: current.clone(),
            time,
            other_players,
            opened_valves: opened_valves.clone(),
        };
        if let Some(flow) = memo.get(&key) {
            return *flow;
        }
        // the point of dynamic programming, when the number of states is much smaller than the
        // sequence of actions you could take, store the states in a memo

        let mut answer = 0;
        let current_closed = !opened_valves.contains(&current);
        let current_flow = *self.flows.get(&current).unwrap() as i64;

        // is the valve at the current position closed and does it have a flow > 0
        // then open valve
        if current_closed && current_flow > 0 {
            //let new_u = u | (1i64 << current_idx);
            opened_valves.push(current.clone());
            answer = answer.max(
                (time - 1) * current_flow 
                    + self.maxflow(current.to_string(), time - 1, other_players, memo, opened_valves.clone()),
            );
        }

        for n in self.valve_map.get(&current).unwrap() {
            answer = answer.max(self.maxflow(
                n.to_string(),
                time - 1,
                other_players,
                memo,
                opened_valves.clone(),
            ));
        }

        let e = memo.entry(key).or_insert(0);
        *e = answer;

        answer
    }
}

fn part_1(input: &str) -> String {
    let layout = parse_input(input);
    //let vis_string = graph_viz_string(&layout);
    //println!("{}", vis_string);

    //println!("flow_rates.len() {}", layout.flows.len());
    //println!("{:?}", &layout.flows);
    let mut memo: HashMap<FancyKey, i64> = HashMap::new();
    let opened_valves = Vec::new();
    let answer = layout.maxflow("AA".to_string(), 30, 0, &mut memo, opened_valves);

    answer.to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    let input = input_txt(InputFile::Example);
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
        assert_eq!(result, "1651");
    }

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
        let result = part_2(&input);
        assert_eq!(result, "1707");
    }

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
        let result = part_1(&input);
        assert_eq!(result, "1792");
    }

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
        let result = part_2(&input);
        assert_eq!(result, "0");
    }
}
