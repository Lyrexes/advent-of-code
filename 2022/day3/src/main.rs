use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("rucksacks")?;
    let reader = BufReader::new(file);
    let rucksacks = reader
        .lines()
        .flatten()
        .map(|x| x.into_bytes())
        .collect::<Vec<Vec<u8>>>();
    let priority_sum = rucksacks.iter().fold(0, |x, s| x + evaluate_priority(&s));
    let priority_badge_sum = rucksacks
        .chunks(3)
        .into_iter()
        .fold(0, |x, s| x + evaluate_priority_badge(s));
    println!("priority_sum: {}", priority_sum);
    println!("priority_badge_sum: {}", priority_badge_sum);
    Ok(())
}

fn evaluate_priority_badge(group_rucksacks: &[Vec<u8>]) -> u32 {
    let first_member_rucksack = group_rucksacks.get(0).unwrap();
    let second_member_rucksack = group_rucksacks.get(1).unwrap();
    let third_member_rucksack = group_rucksacks.get(2).unwrap();
    for item in first_member_rucksack {
        if second_member_rucksack.contains(&item) && third_member_rucksack.contains(&item) {
            return ascii_to_priority(*item);
        }
    }
    unreachable!()
}

fn evaluate_priority(rucksack: &Vec<u8>) -> u32 {
    let mid = rucksack.len() / 2usize;
    let (first_compartment, second_compartment) = rucksack.split_at(mid);
    for item in second_compartment {
        if first_compartment.contains(item) {
            return ascii_to_priority(*item);
        }
    }
    unreachable!()
}

fn ascii_to_priority(code: u8) -> u32 {
    match code {
        b'a'..=b'z' => code as u32 - 96u32,
        b'A'..=b'Z' => code as u32 - 64u32 + 26u32,
        _ => unreachable!(),
    }
}
