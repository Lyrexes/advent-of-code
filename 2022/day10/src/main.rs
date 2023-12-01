use std::{
    fs::File,
    io::{self, BufRead, BufReader}, ops::Rem,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Addx(i32),
    Noop,
}

fn main() -> io::Result<()> {
    let instructions = BufReader::new(File::open("instructions")?)
        .lines()
        .into_iter()
        .flatten()
        .map(|x| parse_instruction(x))
        .collect::<Box<[Instruction]>>();

    println!(
        "signal_strength_sum: {}",
        get_signal_strength_sum(&instructions)
    );
    draw(&instructions);
    Ok(())
}

fn draw(instructions: &Box<[Instruction]>) {
    let mut cycle = 1;
    let mut instruction_in_progress = false;
    let mut current_add_value = 0;
    let mut x_register = 1i32;
    let mut instruction_iter = instructions.iter().peekable();
    let mut screen_pixel = vec![false; 6 * 40].into_boxed_slice();
    let mut sprite_position;
    let mut x_register_crt;
    loop {
        if instruction_iter.peek().is_some() {
            println!("{}", x_register);
            x_register_crt = x_register.rem(40);
            sprite_position = x_register_crt - 1i32..=x_register_crt + 1i32;
            screen_pixel[cycle - 1] = sprite_position.contains(&(cycle as i32 - 1).rem_euclid(40));
        }
        if instruction_in_progress {
            instruction_in_progress = false;
            x_register += current_add_value;
            cycle += 1;
            continue;
        }
        match instruction_iter.next() {
            Some(Instruction::Addx(num)) => {
                instruction_in_progress = true;
                current_add_value = *num;
            }
            Some(Instruction::Noop) => {}
            None => break,
        }
        cycle += 1;
    }
    println!("x: {}", x_register);
    for i in 0..screen_pixel.len() {
        if i.rem_euclid(40) == 0 {
            println!();
        }
        print!("{}", if screen_pixel[i] { "#" } else { "." });
    }
    println!();
}

fn get_signal_strength_sum(instructions: &Box<[Instruction]>) -> i32 {
    let mut cycle = 1;
    let mut instruction_in_progress = false;
    let mut current_add_value = 0;
    let mut x_register = 1;
    let mut signal_strength_sum = 0;
    let mut instruction_iter = instructions.iter();
    loop {
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            signal_strength_sum += x_register * cycle;
        }

        if instruction_in_progress {
            instruction_in_progress = false;
            x_register += current_add_value;
        } else {
            match instruction_iter.next() {
                Some(Instruction::Addx(num)) => {
                    instruction_in_progress = true;
                    current_add_value = *num;
                }
                Some(Instruction::Noop) => {}
                None => break,
            }
        }
        cycle += 1;
    }
    signal_strength_sum
}

fn parse_instruction(raw: String) -> Instruction {
    let mut whitespace_iter = raw.split_ascii_whitespace().into_iter();
    match whitespace_iter.next().unwrap() {
        "addx" => Instruction::Addx(whitespace_iter.next().unwrap().parse::<i32>().unwrap()),
        "noop" => Instruction::Noop,
        _ => unreachable!(),
    }
}
