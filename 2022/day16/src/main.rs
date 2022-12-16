use std::collections::HashMap;


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

    ValveLayout {
        valve_map,
        flows,
    }
}

fn minute() {
    let mut total_pressure_released = 0;
    let open_valves: HashMap<String, usize> = HashMap::new();
    for (_valve, flow) in open_valves.iter() {
        total_pressure_released += flow;
    }

}

struct ValveLayout {
    valve_map: HashMap<String, Vec<String>>,
    flows: HashMap<String, usize>,
}

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

fn part_1(input: &str) -> String {
    let layout = parse_input(input);
    for _ in 0..30 {
        minute();
    }
    let vis_string = graph_viz_string(&layout);
    println!("{}", vis_string);

    "".to_string()
}

fn part_2(input: &str) -> String {
    "".to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    let input = input_txt(InputFile::Real);

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
        assert_eq!(result, "0");
	}

    #[test]
    fn example_part_2() {
        let input = input_txt(InputFile::Example);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "0");
	}

    #[test]
    fn real_part_2() {
        let input = input_txt(InputFile::Real);
		let result = part_2(&input);
        assert_eq!(result, "0");
	}
}
