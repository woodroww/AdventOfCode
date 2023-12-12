use std::collections::HashMap;


#[derive(Debug)]
struct NavNode {
    node_id: String,
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (String, HashMap<String, NavNode>) {
    let mut nodes = HashMap::new();
    let mut lines = input.lines();
    let directions = lines.next().unwrap().to_string();
    lines.next();

    while let Some(line) = lines.next() {
        let mut splitsies = line.split(" = ");
        let node_id = splitsies.next().unwrap().to_string();
        let instructions = splitsies.next().unwrap();
        let mut splitsies = instructions[1..instructions.len() - 1].split(", ");
        let left = splitsies.next().unwrap().to_string();
        let right = splitsies.next().unwrap().to_string(); 
        nodes.insert(node_id.clone(), NavNode { node_id, left, right });
    }

    (directions, nodes)
}

fn part_1(input: &str) -> String {
    let (directions, nodes) = parse_input(input);
    println!("{}", directions);
    println!("{:#?}", nodes);
    let mut step = 0;

    let mut node = nodes.get("AAA").unwrap();
    while node.node_id != "ZZZ" {
        for direction in directions.chars() {
            match direction {
                'L' => {
                    node = nodes.get(&node.left).unwrap();
                },
                'R' => {
                    node = nodes.get(&node.right).unwrap();
                },
                _ => panic!("illegal direction"),
            }
            step += 1;
            if node.node_id == "ZZZ" {
                break;
            }
        }
    }

    println!("step: {} {:?}", step, node);

    step.to_string()
}

fn all_nodes_finished(nodes: &Vec<&NavNode>) -> bool {
    for n in nodes {
        println!("{:?}", n);
        if !n.node_id.ends_with("Z") {
            return false;
        }
    }
    true
}


fn part_2(input: &str) -> String {
    let (directions, nodes) = parse_input(input);

    let mut current_nodes = vec![];
    for (key, node) in &nodes {
        if key.ends_with("A") {
            current_nodes.push(node);
        }
    }

    let mut step = 0;
    while !all_nodes_finished(&current_nodes) {
        for direction in directions.chars() {
            let mut next_nodes = vec![];
            for n in current_nodes.iter() {
                match direction {
                    'L' => {
                        next_nodes.push(nodes.get(&n.left).unwrap());
                    },
                    'R' => {
                        next_nodes.push(nodes.get(&n.right).unwrap());
                    },
                    _ => panic!("illegal direction"),
                }
                // are done?
            }
            current_nodes = next_nodes;
            step += 1;
        }
    }

    step.to_string()
}

fn main() {
    //let input = input_txt(InputFile::Example);
    //let input = std::fs::read_to_string("example2.txt").expect("No example.txt file");
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
        assert_eq!(result, "6");
	}

    #[test]
    fn real_part_1() {
        let input = input_txt(InputFile::Real);
		let result = part_1(&input);
        assert_eq!(result, "20777");
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
