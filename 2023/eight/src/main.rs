use std::collections::HashMap;
use nom::{self, bytes::complete::{take, tag}, IResult, error::Error, sequence::{delimited, separated_pair}};

#[derive(Debug,Clone,PartialEq, PartialOrd,Eq, Ord)]
struct MapNode {
    left:  String,
    right: String
}

fn main() {
    let (instructions_raw, nodes_raw) = include_str!("navigation.input")
        .split_once("\n\n")
        .unwrap();
    let instructions = parse_instructions(instructions_raw);
    let nodes        = parse_nodes(nodes_raw);
    
    println!("steps: {}", get_steps_from_aaa_to_zzz(&instructions, &nodes));
    println!("steps two: {}", get_steps_math(&instructions, &nodes));
}


fn get_steps_math(instructions: &Vec<char>,
                  nodes: &HashMap<String,MapNode>) -> i128 {
    
    let starting_nodes = get_all_xxa(&nodes);
    let minimal_steps_to_xxz = starting_nodes.iter()
        .map(|x| get_steps_from_xxa_to_xxz(instructions, nodes, x))
        .collect::<Vec<i128>>();
    let prime_factors_steps = minimal_steps_to_xxz.iter()
        .map(|&x| get_prime_factors(x))
        .collect::<Vec<Vec<i128>>>();
    
    let mut all_primes = prime_factors_steps.iter()
        .flatten()
        .collect::<Vec<&i128>>();
    all_primes.sort();
    all_primes.dedup();
    all_primes.iter()
        .map(|x| x.pow(get_max_count_in(**x, &prime_factors_steps)))
        .product()
}

fn get_max_count_in(num: i128, factors: &Vec<Vec<i128>>) -> u32{
    factors.iter()
        .map(|x| x.iter().filter(|&&y| y == num).count())
        .max().unwrap() as u32
}


fn get_prime_factors(mut num: i128) -> Vec<i128> {
    let mut factors = vec![];
    for i in 2..num {
        let is_prime = is_prime(i);
        while num.rem_euclid(i) == 0 && is_prime {
            factors.push(i);
            num /= i;
        }
    }
    factors
}

fn is_prime(n: i128) -> bool {
    let limit = (n as f64).sqrt() as i128;

    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}


fn get_steps_from_xxa_to_xxz(instructions: &Vec<char>,
                             nodes: &HashMap<String,MapNode>,
                             starting_node: &str) -> i128 {
    let mut current_node = starting_node;
    let mut step = 0;
    for instruction in instructions.iter().cycle() {
        if current_node.ends_with("Z") {
            break
        }
        current_node = advance_node(current_node, &nodes, *instruction);
        step += 1;
    }
    step
}

fn advance_node<'a>(current_node: &'a str, 
                        nodes: &'a HashMap<String,MapNode>,
                        direction: char) -> &'a str {
    if direction == 'L' {
        &nodes[&current_node as &str].left.as_str()  
    } else {
        &nodes[&current_node as &str].right.as_str()  
    }
}


fn get_all_xxa(nodes: &HashMap<String,MapNode>) -> Vec<&str> {
    nodes.iter()
        .map(|(x,_)| x.as_str())
        .filter(|x| x.ends_with("A"))
        .collect()
}

fn is_all_xxz(nodes: &Vec<&str>) -> bool {
    nodes.iter().all(|x| x.ends_with("Z"))
}

fn get_steps_from_aaa_to_zzz(instructions: &Vec<char>,
                             nodes: &HashMap<String,MapNode>) -> u64 {
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
    counter
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
