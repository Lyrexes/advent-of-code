use std::collections::HashMap;
use std::collections::HashSet;
use std::{thread, time};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Node {
    Start,
    End,
    Path(u8),
}

fn main() {
    let map = include_str!("map")
        .lines()
        .map(parse_nodes)
        .map(Vec::into_boxed_slice)
        .collect::<Box<[Box<[Node]>]>>();

    let start = get_node(&map, Node::Start);
    let end = get_node(&map, Node::End);
    println!("distance: {}", find_shortest_path(start, end, &map));
    println!("finde find_shortes_beginning: {}", find_shortes_beginning(start, end, &map));
}

fn find_shortes_beginning(
    start: (usize, usize),
    end: (usize, usize),
    map: &Box<[Box<[Node]>]>,
) -> u32 {
    let mut starting_points = vec![start];
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if let Node::Path(0) = map[row][column] {
                starting_points.push((row, column));
            }
        }
    }
    println!("starting_points {:?}", starting_points.len());
    starting_points
        .into_iter()
        .map(|s| find_shortest_path(s, end, &map))
        .min()
        .unwrap()
}

fn find_shortest_path(
    start: (usize, usize),
    end: (usize, usize),
    nodes: &Box<[Box<[Node]>]>,
) -> u32 {
    let mut open_set = vec![start];
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut distance_to_node = init_cheapest_path_to(nodes, start);
    let mut total_distance_with_node = init_cheapest_path_through(nodes, start, end);
    let mut current_neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);
    let mut current_node;
    let mut new_distance_to_neighbor;
    let mut visited = HashSet::new();
    while !open_set.is_empty() {
        current_node = open_set.remove(0);
        visited.insert(current_node);
        if current_node == end {
            return distance_to_node[&came_from[&end]] + 1;
        }
        current_neighbors = get_neighbors(current_neighbors, current_node, nodes);
        for neighbor in current_neighbors.iter() {
            new_distance_to_neighbor = distance_to_node[&current_node] + 1;
            if new_distance_to_neighbor <= distance_to_node[&neighbor] {
                came_from.insert(*neighbor, current_node);
                *distance_to_node.get_mut(neighbor).unwrap() =
                    new_distance_to_neighbor;
                *total_distance_with_node.get_mut(neighbor).unwrap() =
                    new_distance_to_neighbor + get_distance_between(*neighbor, end);
                if !open_set.contains(neighbor) {
                    open_set.push(*neighbor);
                }
            }
        }
        open_set.sort_by(|a, b| {
            total_distance_with_node[a].cmp(&total_distance_with_node[b])
        });
    }
    u32::MAX
}

fn print_map(visited: &HashSet<(usize, usize)>, map: &Box<[Box<[Node]>]>) {
    for row in 0..map.len() {
        for column in 0..map.first().unwrap().len() {
            if visited.contains(&(row, column)) {
                print!("+");
            } else {
                print!(
                    "{}",
                    match map[row][column] {
                        Node::Start => 'S',
                        Node::End => 'E',
                        Node::Path(h) => (h + b'a') as char,
                    }
                );
            }
        }
        println!();
    }
}

fn get_neighbors(
    mut neighbors: Vec<(usize, usize)>,
    node: (usize, usize),
    nodes: &Box<[Box<[Node]>]>,
) -> Vec<(usize, usize)> {
    let current_node = nodes[node.0][node.1];
    neighbors.clear();

    if node.0 + 1 < nodes.len()
        && get_height_diffrence(nodes[node.0 + 1][node.1], current_node) < 2
    {
        neighbors.push((node.0 + 1, node.1));
    }
    if node.1 + 1 < nodes[0].len()
        && get_height_diffrence(nodes[node.0][node.1 + 1], current_node) < 2
    {
        neighbors.push((node.0, node.1 + 1));
    }
    if node.0 > 0
        && get_height_diffrence(nodes[node.0 - 1][node.1], current_node) < 2
    {
        neighbors.push((node.0 - 1, node.1));
    }
    if node.1 > 0
        && get_height_diffrence(nodes[node.0][node.1 - 1], current_node) < 2
    {
        neighbors.push((node.0, node.1 - 1));
    }
    neighbors
}

fn get_height_diffrence(lhs: Node, rhs: Node) -> i16 {
    let lhs_height = match lhs {
        Node::Start => 0u8,
        Node::End => 25u8,
        Node::Path(h) => h,
    };
    let rhs_height = match rhs {
        Node::Start => 0u8,
        Node::End => 25u8,
        Node::Path(h) => h,
    };
    lhs_height as i16 - rhs_height as i16
}

fn init_cheapest_path_to(
    nodes: &Box<[Box<[Node]>]>,
    start: (usize, usize),
) -> HashMap<(usize, usize), u32> {
    let mut cheapest_path_to = HashMap::new();
    for row in 0..nodes.len() {
        for column in 0..nodes[0].len() {
            if start == (row, column) {
                cheapest_path_to.insert(start, 0);
                continue;
            }
            cheapest_path_to.insert((row, column), u32::MAX);
        }
    }
    cheapest_path_to
}

fn init_cheapest_path_through(
    nodes: &Box<[Box<[Node]>]>,
    start: (usize, usize),
    end: (usize, usize),
) -> HashMap<(usize, usize), u32> {
    let mut cheapest_path_through = HashMap::new();
    for row in 0..nodes.len() {
        for column in 0..nodes[0].len() {
            if start == (row,column) {
                cheapest_path_through
                    .insert(start, get_distance_between(start, end));
                continue;
            }
            cheapest_path_through.insert((row, column), u32::MAX);
        }
    }
    cheapest_path_through
}

fn get_distance_between(node: (usize, usize), end: (usize, usize)) -> u32 {
    get_distance((
        usize::abs_diff(node.0, end.0),
        usize::abs_diff(node.1, end.1),
    ))
}

fn get_distance(node: (usize, usize)) -> u32 {
    f64::sqrt((node.0 * node.0 + node.1 * node.1) as f64).round() as u32
}

fn get_node(map: &Box<[Box<[Node]>]>, node: Node) -> (usize, usize) {
    for row in 0..map.len() {
        for column in 0..map.first().unwrap().len() {
            if map[row][column] == node {
                return (row, column);
            }
        }
    }
    unreachable!()
}

fn parse_nodes(raw: &str) -> Vec<Node> {
    raw.bytes()
        .map(|x| match x {
            b'S' => Some(Node::Start),
            b'E' => Some(Node::End),
            b'a'..=b'z' => Some(Node::Path(x - b'a')),
            _ => None,
        })
        .flatten()
        .collect::<Vec<Node>>()
}
