use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct Instruction {
    quantity: u8,
    src: usize,
    dest: usize,
}

fn main() -> io::Result<()> {
    let crane_file = File::open("crane")?;
    let instructions_file = File::open("instructions")?;
    let instructions = parse_instructions(instructions_file);
    let cargo = parse_crane(crane_file);
    print_top_row(execute_instructions_9000(&cargo, &instructions));
    print_top_row(execute_instructions_9001(&cargo, &instructions));
    Ok(())
}

fn execute_instructions_9000(
    cargo_: &Vec<Vec<u8>>,
    instructions: &Vec<Instruction>,
) -> Vec<Vec<u8>> {
    let mut cargo = cargo_.clone();
    for instruction in instructions.iter() {
        let mut current_cargo;
        for _ in 0..instruction.quantity {
            current_cargo = cargo[instruction.src].pop().unwrap();
            cargo[instruction.dest].push(current_cargo);
        }
    }
    cargo
}

fn execute_instructions_9001(
    cargo_: &Vec<Vec<u8>>,
    instructions: &Vec<Instruction>,
) -> Vec<Vec<u8>> {
    let mut cargo = cargo_.clone();
    let mut src_size;
    let mut current_cargo;
    for instruction in instructions.iter() {
        src_size = cargo[instruction.src].len();
        current_cargo = cargo[instruction.src]
            .drain((src_size - instruction.quantity as usize)..)
            .collect::<Vec<u8>>();
        cargo[instruction.dest].append(&mut current_cargo);
    }
    cargo
}

fn print_top_row(cargo: Vec<Vec<u8>>) {
    println!(
        "top row: {}",
        cargo
            .iter()
            .map(|x| std::str::from_utf8(&x))
            .flatten()
            .map(|x| x.chars().rev().nth(0))
            .flatten()
            .collect::<String>()
    );
}

fn parse_instructions(file: File) -> Vec<Instruction> {
    let instructions_reader = BufReader::new(file);
    let instructions = instructions_reader
        .lines()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
    instructions
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<u8>())
                .flatten()
                .collect::<Vec<u8>>()
        })
        .map(|x| Instruction {
            quantity: x[0],
            src: (x[1] - 1) as usize,
            dest: (x[2] - 1) as usize,
        })
        .collect()
}

fn parse_crane(file: File) -> Vec<Vec<u8>> {
    let crane_reader = BufReader::new(file);
    let mut crane_stacks = crane_reader
        .lines()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
    crane_stacks.reverse();
    parse_crane_stacks(&crane_stacks)
}

fn parse_crane_stacks(raw_crane: &[String]) -> Vec<Vec<u8>> {
    let (stacks_amount_raw, stacks_raw) = raw_crane.split_first().unwrap();
    let stacks_amount = stacks_amount_raw.split_ascii_whitespace().count();
    let mut stacks: Vec<Vec<u8>> = vec![Vec::new(); stacks_amount];
    let mut cargo_indices: Vec<(u8, usize)>;
    for stack in stacks_raw {
        cargo_indices = stack
            .match_indices(|x| char::is_ascii_alphabetic(&x))
            .map(|x| (*x.1.as_bytes().get(0).unwrap(), x.0 / 4usize))
            .collect();
        for (cargo, i) in cargo_indices.iter() {
            stacks.get_mut(*i).unwrap().push(*cargo);
        }
    }
    stacks
}
