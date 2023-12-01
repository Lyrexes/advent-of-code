use std::{iter::Peekable, str::Bytes};

#[derive(Debug, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}

impl std::fmt::Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(x) => {
                write!(f, "[")?;
                for i in 0..x.len() {
                    if i + 1 == x.len() {
                        x[i].fmt(f)?;
                    } else {
                        x[i].fmt(f)?;
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
            Packet::Integer(x) => write!(f, "{}", x),
        }
    }
}

impl PartialEq for Packet {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Packet::List(lhs), Packet::List(rhs)) => {
                if lhs.len() != rhs.len() {
                    return false;
                }
                for i in 0..lhs.len() {
                    if !lhs[i].eq(&rhs[i]) {
                        return false;
                    }
                }
                true
            }
            (x @ Packet::List(_), y @ Packet::Integer(_)) => x.eq(&Packet::List(vec![y.clone()])),
            (x @ Packet::Integer(_), y @ Packet::List(_)) => Packet::List(vec![x.clone()]).eq(y),
            (Packet::Integer(x), Packet::Integer(y)) => x == y,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::List(lhs), Packet::List(rhs)) => {
                for i in 0..rhs.len() {
                    if i >= lhs.len() {
                        return Some(std::cmp::Ordering::Less);
                    }
                    match lhs[i].partial_cmp(&rhs[i]) {
                        x @ Some(std::cmp::Ordering::Greater) => return x,
                        y @ Some(std::cmp::Ordering::Less) => return y,
                        _ => {}
                    }
                }
                if lhs.len() > rhs.len() {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    Some(std::cmp::Ordering::Equal)
                }
            }
            (x @ Packet::List(_), y @ Packet::Integer(_)) => {
                x.partial_cmp(&Packet::List(vec![y.clone()]))
            }
            (x @ Packet::Integer(_), y @ Packet::List(_)) => {
                Packet::List(vec![x.clone()]).partial_cmp(y)
            }
            (Packet::Integer(x), Packet::Integer(y)) => x.partial_cmp(y),
        }
    }
}

fn main() {
    println!("index sum right order: {}", get_index_sum_right_order());
    println!("protocol devider: {}", get_protocol_deviders_index_product());
}

fn get_protocol_deviders_index_product() -> usize {
    let mut packets = include_str!("packets")
        .split("\n\n")
        .map(parse_packet_pair)
        .map(|(a, b)| [a, b])
        .flatten()
        .collect::<Vec<Packet>>();
    let (first_protocol_devider, second_protocol_devider) = parse_packet_pair("[[2]]\n[[6]]");
    packets.push(first_protocol_devider.clone());
    packets.push(second_protocol_devider.clone());
    packets.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (packets
        .iter()
        .position(|x| *x == first_protocol_devider)
        .unwrap()
        + 1)
        * (packets
            .iter()
            .position(|x| *x == second_protocol_devider)
            .unwrap()
            + 1)
}

fn get_index_sum_right_order() -> usize {
    let packet_pairs = include_str!("packets")
        .split("\n\n")
        .map(parse_packet_pair)
        .collect::<Vec<(Packet, Packet)>>();
    packet_pairs
        .into_iter()
        .enumerate()
        .map(|(i, (lhs, rhs))| if lhs < rhs { Some(i + 1) } else { None })
        .flatten()
        .fold(0, |acc, x| acc + x)
}

fn parse_packet_pair(raw: &str) -> (Packet, Packet) {
    let mut pair_iterator = raw.lines();
    let mut first_packet = pair_iterator.next().unwrap().bytes().peekable();
    let mut second_packet = pair_iterator.next().unwrap().bytes().peekable();
    (
        parse_packet(&mut first_packet),
        parse_packet(&mut second_packet),
    )
}

fn parse_packet(raw: &mut Peekable<Bytes>) -> Packet {
    while let Some(ch) = raw.peek() {
        match ch {
            b'[' => return parse_list(raw),
            b'0'..=b'9' => return parse_number(raw),
            _ => {
                raw.next();
            }
        }
    }
    unreachable!()
}

fn parse_list(raw: &mut Peekable<Bytes>) -> Packet {
    let mut list = vec![];
    raw.next();
    while let Some(ch) = raw.peek() {
        match ch {
            b']' => {
                raw.next();
                return Packet::List(list);
            }
            _ => list.push(parse_packet(raw)),
        }
    }
    unreachable!()
}

fn parse_number(raw: &mut Peekable<Bytes>) -> Packet {
    let mut num = "".to_string();
    while let Some(ch) = raw.peek() {
        match ch {
            b'0'..=b'9' => num.push(raw.next().unwrap() as char),
            _ => return Packet::Integer(num.parse().unwrap()),
        }
    }
    unreachable!()
}
