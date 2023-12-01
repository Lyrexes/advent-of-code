use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{AddAssign, MulAssign, Rem},
};

const MODS: [u8; 8] = [11, 5, 7, 2, 17, 13, 3, 19];

#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus(u32),
    Multiply(u32),
    Squareroot,
}

#[derive(Debug, Clone)]
struct Monkey {
    worry_level: Vec<u128>,
    inspect_op: Operation,
    worry_test: u128,
    true_mokey: usize,
    false_monkey: usize,
}

#[derive(Debug, Clone, Copy)]
struct WorryLevel {
    data: [u8; 8],
}

#[derive(Debug, Clone)]
struct Monkey2 {
    worry_level: Vec<WorryLevel>,
    inspect_op: Operation,
    worry_test: u128,
    true_mokey: usize,
    false_monkey: usize,
}

impl WorryLevel {
    fn is_divisble(&self, div: u128) -> bool {
        let index = MODS.iter().position(|&x| div == x as u128).unwrap();
        self.data[index] == 0
    }
}

impl MulAssign<WorryLevel> for WorryLevel {
    fn mul_assign(&mut self, rhs: WorryLevel) {
        for (i_lhs, lhs) in self.data.iter_mut().enumerate() {
            *lhs = (*lhs as u128 * rhs.data[i_lhs] as u128).rem_euclid(MODS[i_lhs] as u128) as u8;
        }
    }
}

impl MulAssign<u128> for WorryLevel {
    fn mul_assign(&mut self, rhs: u128) {
        for (i, el) in self.data.iter_mut().enumerate() {
            *el =
                ((*el as u128) * rhs.rem_euclid(MODS[i] as u128)).rem_euclid(MODS[i] as u128) as u8;
        }
    }
}

impl AddAssign<u128> for WorryLevel {
    fn add_assign(&mut self, rhs: u128) {
        for (i, el) in self.data.iter_mut().enumerate() {
            *el =
                ((*el as u128) + rhs.rem_euclid(MODS[i] as u128)).rem_euclid(MODS[i] as u128) as u8;
        }
    }
}

impl From<u128> for WorryLevel {
    fn from(item: u128) -> Self {
        let mut data = [0; 8];
        for (i, el) in data.iter_mut().enumerate() {
            *el = item.rem_euclid(MODS[i] as u128) as u8;
        }
        WorryLevel { data }
    }
}

fn main() -> io::Result<()> {
    let lines = BufReader::new(File::open("monkeys")?)
        .lines()
        .flatten()
        .collect::<Vec<String>>();
    let monkeys = lines
        .chunks(7)
        .map(|x| parse_monkey(x))
        .collect::<Box<[Monkey]>>();

    let monkeys2 = lines
        .chunks(7)
        .map(|x| parse_monkey2(x))
        .collect::<Box<[Monkey2]>>();

    println!(
        "monkey buisiness: {}",
        get_monkey_business(monkeys.to_owned())
    );
    println!(
        "monkey business without worry relief: {}",
        get_monkey_business_without_worry_relief(monkeys.to_owned())
    );
    println!(
        "monkey business without worry relief version 2: {}",
        get_monkey_business_without_worry_relief2(monkeys2.to_owned())
    );
    Ok(())
}

fn parse_monkey2(raw: &[String]) -> Monkey2 {
    Monkey2 {
        worry_level: raw[1]
            .split_ascii_whitespace()
            .into_iter()
            .map(|x| {
                if x.ends_with(',') {
                    str::parse::<u128>(x.trim_matches(','))
                } else {
                    str::parse::<u128>(x)
                }
            })
            .flatten()
            .map(WorryLevel::from)
            .collect::<Vec<WorryLevel>>(),
        inspect_op: parse_operation(&raw[2]),
        worry_test: raw[3]
            .split_ascii_whitespace()
            .nth_back(0)
            .map(str::parse::<u128>)
            .unwrap()
            .unwrap(),
        true_mokey: raw[4]
            .split_ascii_whitespace()
            .nth_back(0)
            .map(str::parse::<usize>)
            .unwrap()
            .unwrap(),
        false_monkey: raw[5]
            .split_ascii_whitespace()
            .nth_back(0)
            .map(str::parse::<usize>)
            .unwrap()
            .unwrap(),
    }
}

fn parse_monkey(raw: &[String]) -> Monkey {
    Monkey {
        worry_level: raw[1]
            .split_ascii_whitespace()
            .into_iter()
            .map(|x| {
                if x.ends_with(',') {
                    str::parse::<u128>(x.trim_matches(','))
                } else {
                    str::parse::<u128>(x)
                }
            })
            .flatten()
            .collect::<Vec<u128>>(),
        inspect_op: parse_operation(&raw[2]),
        worry_test: raw[3]
            .split_ascii_whitespace()
            .nth_back(0)
            .map(str::parse::<u128>)
            .unwrap()
            .unwrap(),
        true_mokey: raw[4]
            .split_ascii_whitespace()
            .nth_back(0)
            .map(str::parse::<usize>)
            .unwrap()
            .unwrap(),
        false_monkey: raw[5]
            .split_ascii_whitespace()
            .nth_back(0)
            .map(str::parse::<usize>)
            .unwrap()
            .unwrap(),
    }
}

fn get_monkey_business_without_worry_relief2(mut monkeys: Box<[Monkey2]>) -> u128 {
    let mut inspect_count: Vec<u128> = vec![0; 8];
    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            for item_index in 0..monkeys[monkey_index].worry_level.len() {
                match monkeys[monkey_index].inspect_op {
                    Operation::Multiply(m) => {
                        monkeys[monkey_index].worry_level[item_index] *= m as u128;
                    }
                    Operation::Plus(p) => {
                        monkeys[monkey_index].worry_level[item_index] += p as u128;
                    }
                    Operation::Squareroot => {
                        let num = monkeys[monkey_index].worry_level[item_index];
                        monkeys[monkey_index].worry_level[item_index] *= num;
                    }
                }

                if monkeys[monkey_index].worry_level[item_index]
                    .is_divisble(monkeys[monkey_index].worry_test)
                {
                    monkeys[monkeys[monkey_index].true_mokey]
                        .worry_level
                        .push(monkeys[monkey_index].worry_level[item_index]);
                } else {
                    monkeys[monkeys[monkey_index].false_monkey]
                        .worry_level
                        .push(monkeys[monkey_index].worry_level[item_index]);
                }
            }
            inspect_count[monkey_index] += monkeys[monkey_index].worry_level.len() as u128;
            monkeys[monkey_index].worry_level.clear();
        }
    }

    inspect_count.sort_by(|a, b| b.cmp(a));
    inspect_count[0] * inspect_count[1]
}

fn get_monkey_business_without_worry_relief(mut monkeys: Box<[Monkey]>) -> u128 {
    let mut inspect_count: Vec<u128> = vec![0; 8];
    let all_mods: u128 = monkeys.iter().fold(1, |acc, x| acc * x.worry_test);
    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            for item_index in 0..monkeys[monkey_index].worry_level.len() {
                match monkeys[monkey_index].inspect_op {
                    Operation::Multiply(m) => {
                        monkeys[monkey_index].worry_level[item_index] *=
                            (m as u128).rem_euclid(all_mods);
                        monkeys[monkey_index].worry_level[item_index] =
                            monkeys[monkey_index].worry_level[item_index].rem_euclid(all_mods)
                                as u128;
                    }
                    Operation::Plus(p) => {
                        monkeys[monkey_index].worry_level[item_index] +=
                            (p as u128).rem_euclid(all_mods);
                        monkeys[monkey_index].worry_level[item_index] =
                            monkeys[monkey_index].worry_level[item_index].rem_euclid(all_mods);
                    }
                    Operation::Squareroot => {
                        monkeys[monkey_index].worry_level[item_index] *=
                            monkeys[monkey_index].worry_level[item_index];
                        monkeys[monkey_index].worry_level[item_index] =
                            monkeys[monkey_index].worry_level[item_index].rem_euclid(all_mods);
                    }
                }

                if monkeys[monkey_index].worry_level[item_index]
                    .rem(monkeys[monkey_index].worry_test)
                    == 0
                {
                    monkeys[monkeys[monkey_index].true_mokey]
                        .worry_level
                        .push(monkeys[monkey_index].worry_level[item_index]);
                } else {
                    monkeys[monkeys[monkey_index].false_monkey]
                        .worry_level
                        .push(monkeys[monkey_index].worry_level[item_index]);
                }
            }
            inspect_count[monkey_index] += monkeys[monkey_index].worry_level.len() as u128;
            monkeys[monkey_index].worry_level.clear();
        }
    }

    inspect_count.sort_by(|a, b| b.cmp(a));
    inspect_count[0] * inspect_count[1]
}

fn get_monkey_business(mut monkeys: Box<[Monkey]>) -> i32 {
    let mut inspect_count = vec![0; 8];
    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for item_index in 0..monkeys[monkey_index].worry_level.len() {
                match monkeys[monkey_index].inspect_op {
                    Operation::Multiply(m) => {
                        monkeys[monkey_index].worry_level[item_index] *= m as u128
                    }
                    Operation::Plus(p) => {
                        monkeys[monkey_index].worry_level[item_index] += p as u128
                    }
                    Operation::Squareroot => {
                        monkeys[monkey_index].worry_level[item_index] *=
                            monkeys[monkey_index].worry_level[item_index]
                    }
                }

                monkeys[monkey_index].worry_level[item_index] /= 3;

                if monkeys[monkey_index].worry_level[item_index]
                    .rem(monkeys[monkey_index].worry_test)
                    == 0
                {
                    monkeys[monkeys[monkey_index].true_mokey]
                        .worry_level
                        .push(monkeys[monkey_index].worry_level[item_index]);
                } else {
                    monkeys[monkeys[monkey_index].false_monkey]
                        .worry_level
                        .push(monkeys[monkey_index].worry_level[item_index]);
                }
            }
            inspect_count[monkey_index] += monkeys[monkey_index].worry_level.len();
            monkeys[monkey_index].worry_level.clear();
        }
    }

    inspect_count.sort_by(|a, b| b.cmp(a));
    (inspect_count[0] * inspect_count[1]) as i32
}

fn parse_operation(raw: &String) -> Operation {
    let mut white_space_iter = raw.split_ascii_whitespace();
    let num = white_space_iter.next_back().map(str::parse::<u32>).unwrap();
    let op = white_space_iter
        .next_back()
        .map(|x| x.chars().nth(0))
        .unwrap();
    match (op, num) {
        (Some('*'), Ok(x)) => Operation::Multiply(x),
        (Some('+'), Ok(x)) => Operation::Plus(x),
        (Some('*'), Err(_)) => Operation::Squareroot,
        _ => unreachable!(),
    }
}
