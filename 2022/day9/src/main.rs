use std::collections::HashSet;
use std::str::FromStr;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

fn main() -> io::Result<()> {
    let instructions = BufReader::new(File::open("instructions")?)
        .lines()
        .into_iter()
        .flatten()
        .map(|x| parse_moves(x))
        .flatten()
        .collect::<Vec<Direction>>();

    println!(
        "Tail unique positions visited with 1 knot: {}",
        simulate_rope_simple(&instructions)
    );

    println!(
        "Tail unique positions visited with 9 knots: {}",
        simulate_rope_nine_knots(&instructions)
    );

    Ok(())
}

fn simulate_rope_nine_knots(instructions: &Vec<Direction>) -> usize {
    let mut positions = HashSet::new();
    let mut knot_positions = [(0i16, 0i16); 10];
    let mut current_head;
    let mut current_tail;
    positions.insert(knot_positions[9]);

    for instruction in instructions.iter() {
            knot_positions[0] = move_head(*instruction, knot_positions[0]);
        for knot_index in 0..9 {
            current_head = knot_positions[knot_index];
            current_tail = knot_positions[knot_index+1];
            if tail_needs_to_move(current_tail, current_head) {
                current_tail = move_tail_to_head(current_tail, current_head);
                if knot_index == 8 {
                    positions.insert(current_tail);
                }
            }
            knot_positions[knot_index+1] = current_tail;
        }
    }
    positions.len()
}

fn simulate_rope_simple(instructions: &Vec<Direction>) -> usize {
    let mut positions = HashSet::new();
    let mut head_position = (0i16, 0i16);
    let mut tail_position = (0i16, 0i16);
    positions.insert(tail_position);

    for instruction in instructions.iter() {
        head_position = move_head(*instruction, head_position);
        if tail_needs_to_move(tail_position, head_position) {
            tail_position = move_tail_to_head(tail_position, head_position);
            positions.insert(tail_position);
        }
    }
    positions.len()
}

fn parse_moves(raw: String) -> Vec<Direction> {
    let mut whitespace_iter = raw.split_ascii_whitespace().into_iter();
    let direction = whitespace_iter.next().unwrap();
    let distance = whitespace_iter.next().unwrap().parse::<usize>().unwrap();
    match direction.bytes().nth(0).unwrap() {
        b'R' => vec![Direction::Right; distance],
        b'U' => vec![Direction::Up; distance],
        b'L' => vec![Direction::Left; distance],
        b'D' => vec![Direction::Down; distance],
        _ => unreachable!(),
    }
}

fn move_head(direction: Direction, head_position: (i16, i16)) -> (i16, i16) {
    let mut new_position = head_position;
    match direction {
        Direction::Right => new_position.0 = head_position.0 + 1,
        Direction::Left => new_position.0 = head_position.0 - 1,
        Direction::Up => new_position.1 = head_position.1 + 1,
        Direction::Down => new_position.1 = head_position.1 - 1,
    }
    new_position
}

fn tail_needs_to_move(tail_position: (i16, i16), head_position: (i16, i16)) -> bool {
    let mut tail_needs_move = true;
    for x in -1..=1 {
        for y in -1..=1 {
            tail_needs_move &= (tail_position.0 + x, tail_position.1 + y) != head_position;
        }
    }
    tail_needs_move
}

fn move_tail_to_head(tail_position: (i16, i16), head_position: (i16, i16)) -> (i16, i16) {
    let mut new_tail_position;
    let mut tail_position_result = (0i16,0i16);
    for x in -1..=1 {
        for y in -1..=1 {
            new_tail_position = (tail_position.0 + x, tail_position.1 + y);
            if !tail_needs_to_move(new_tail_position, head_position)
                && (new_tail_position.0 == head_position.0
                    || new_tail_position.1 == head_position.1)
            {
                return new_tail_position;
            } else if !tail_needs_to_move(new_tail_position, head_position) {
               tail_position_result = new_tail_position;
            }
        }
    }
    tail_position_result
}
