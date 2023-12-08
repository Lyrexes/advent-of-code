use std::{rc::Rc, cell::RefCell, collections::HashMap};
use nom::{self, bytes::complete::{take, tag}, IResult, error::Error, sequence::{delimited, separated_pair}};

#[derive(Debug,Clone,PartialEq, PartialOrd,Eq, Ord)]
struct MapNode {
    left:  String,
    right: String
}

type MapNodeRef = Rc<RefCell<MapNode>>;


fn main() {
    let (instructions_raw, nodes_raw) = include_str!("navigation.input")
        .split_once("\n\n")
        .unwrap();
    let instructions = parse_instructions(instructions_raw);
    let nodes        = parse_nodes(nodes_raw);
    
    let mut current_node = "AAA";
    let mut counter = 0;
    for instruction in instructions.iter().cycle() {
        if current_node == "ZZZ" {
            break;
        }
        if *instruction == 'L' {
            current_node = &nodes[&current_node as &str].left.as_str();
        } else {
            current_node = &nodes[&current_node as &str].right.as_str();
        }
        counter += 1;
    }

    println!("steps: {}", counter);
}

fn parse_instructions(raw: &str) -> Vec<char> {
    raw.chars().filter(|x| x.is_alphanumeric()).collect()
}

fn parse_nodes(raw: &str) -> HashMap<String,MapNode> {
    raw.split("\n")
        .filter(|x| !x.is_empty())
        .map(parse_node)
        .collect()
}

fn take3(raw: &str) -> IResult<&str,&str> {
    take::<_,_,Error<_>>(3usize)(raw)
}

fn parse_node(raw: &str) -> (String,MapNode) {
    let (mut rem, identifier) = take3(raw).unwrap();
    (rem, _) = take3(rem).unwrap();
    let (_, (left, right)) = delimited(
        tag("("),  
        separated_pair(take3, tag(", "), take3),
        tag(")")
    )(rem).unwrap();

    (
        identifier.to_string(),
        MapNode {
            left: left.to_string(),
            right: right.to_string()
        }
    )
}
