use std::collections::HashMap;

use libaoc::{Answer, Solution};

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Haunted Wasteland"
    }

    /// Start a "AAA" and follow links until "ZZZ" is eventually found
    fn part1(&self, input: &str) -> Answer {
        let map = parse(input);
        let mut counter = 0;
        let mut current_node = "AAA";

        loop {
            current_node = map.get(current_node, counter);
            counter += 1;
            if current_node == "ZZZ" {
                break;
            }
        }

        counter.into()
    }

    fn part2(&self, input: &str) -> Answer {
        let map = parse(input);

        // Find all positions that ends with 'A'. From there we will check all of them to see when
        // they reach a node ending in 'Z'
        let mut nodes = Vec::new();
        for &id in map.nodes.keys() {
            if id.ends_with('A') {
                nodes.push(id);
            }
        }

        // Find all the cycles for each of the nodes that end with "A"
        let mut cycles = Vec::new();
        for mut current_node in nodes {
            let mut index = 0;
            loop {
                current_node = map.get(current_node, index);
                index += 1;
                if current_node.ends_with('Z') {
                    cycles.push(index);
                    break;
                }
            }
        }

        // This is where I got stuck. I fould out that I have to apply the "Least Common Multiple"
        // to the list of cycles. For this I found the `num` crate as this function thanks!
        cycles.into_iter().reduce(num::integer::lcm).unwrap().into()
    }
}

struct Map<'a> {
    /// Char array of b'L' and b'R'
    instructions: &'a [u8],
    /// Node (Left, Right)
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
    instruction_size: usize,
}

impl<'a> Map<'a> {
    fn new(instructions: &'a [u8], nodes: HashMap<&'a str, (&'a str, &'a str)>) -> Self {
        Self {
            instructions,
            nodes,
            instruction_size: instructions.len(),
        }
    }

    fn get(&self, key: &'a str, index: usize) -> &'a str {
        let (left, right) = self.nodes.get(key).expect("There should be an entry {key}");

        // Need to wrap index as this might go past the instruction list as we might not have found
        // the result yet
        match self.instructions[index % self.instruction_size] as char {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Map {
    let (instructions, node_list) = input.split_once("\n\n").unwrap();

    let mut nodes = HashMap::new();
    for node in node_list.lines() {
        let (id, children) = node.split_once(" = ").unwrap();
        let children = children
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();
        nodes.insert(id, children);
    }

    Map::new(instructions.as_bytes(), nodes)
}
