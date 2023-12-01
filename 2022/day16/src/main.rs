use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
    id: (u8, u8),
    flow_rate: u8,
    neigbors: Vec<(u8, u8)>,
}

#[derive(Debug,Clone, Copy)]
struct Node {
    node_id: (u8,u8),
    bitmask: u16,
    cost: u16
}

fn main() {
    let valves = include_str!("valve-test")
        .lines()
        .map(parse_valve)
        .collect::<Vec<Valve>>();
    pretty_print_valves(&valves);
    


}

fn init_distance(valves: &Vec<Valve>) -> HashMap<((u8,u8), u16), u16>{
    let mut distance = HashMap::new();
    for (index,valve) in valves.iter().enumerate() {
        if index == 0 {
            distance.insert((valve.id, 0), 0);
            continue;
        } 
        for i in 0..15 {
            distance.insert((valve.id, (1 << i)), u16::MAX);
        }
    }
    distance
}

fn pretty_print_valves(valves: &Vec<Valve>) {
    for valve in valves.iter() {
        print!(
            "name: {}{}, flow rate: {}",
            valve.id.0 as char,
            valve.id.1 as char,
            valve.flow_rate,
        );
        print!(", neighbors: [");
        for (i,neigbor) in valve.neigbors.iter().enumerate() {
            if i != valve.neigbors.len() - 1 {
                print!(
                    "{}{}, ",
                    neigbor.0 as char,
                    neigbor.1 as char,
                );
            } else {
                print!(
                    "{}{}]",
                    neigbor.0 as char,
                    neigbor.1 as char,
                );
            }
        }
        println!();
    }
}


fn parse_valve(raw: &str) -> Valve {
    let (valve, neigbors_raw) = raw.split_once(';').unwrap();
    let (name, flow_rate_raw) = valve.split_once('=').unwrap();
    let flow_rate = flow_rate_raw.parse::<u8>().unwrap();
    let id = name
        .split_ascii_whitespace()
        .filter(|&x| x.chars().all(|ch| ch.is_uppercase()))
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let neigbors = neigbors_raw
        .split_ascii_whitespace()
        .map(|x| x.trim_matches(','))
        .filter(|&x| x.chars().all(|ch| ch.is_uppercase()))
        .map(|x| (x.bytes().nth(0).unwrap(), x.bytes().nth(1).unwrap()))
        .collect::<Vec<(u8, u8)>>();

    Valve {
        id: (id[0] as u8, id[1] as u8),
        flow_rate,
        neigbors,
    }
}
