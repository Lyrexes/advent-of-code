use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Range;

fn main() -> io::Result<()> {
    let file = File::open("assignments")?;
    let reader = BufReader::new(file);
    let assignments = reader
        .lines()
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();
    let fully_contained_count = assignments
        .iter()
        .map(|x| parse_assignment_pair(x))
        .fold(0, |acc, r| acc + is_contained(r) as u32);
    let overlapping_count = assignments
        .iter()
        .map(|x| parse_assignment_pair(x))
        .fold(0, |acc, r| acc + is_overlapping(r) as u32);
    println!("fully contained pairs: {}", fully_contained_count);
    println!("overlapping pairs: {}", overlapping_count);
    Ok(())
}

fn parse_assignment_pair(raw: &String) -> (Range<u32>, Range<u32>) {
    let mut elf_iter = raw.split(|x| x == ',');
    let first_elf = elf_iter.next().unwrap();
    let second_elf = elf_iter.next().unwrap();
    let mut first_range = first_elf.split('-');
    let mut second_range = second_elf.split('-');
    (
        Range {
            start: first_range.next().unwrap().parse::<u32>().unwrap(),
            end: first_range.next().unwrap().parse::<u32>().unwrap(),
        },
        Range {
            start: second_range.next().unwrap().parse::<u32>().unwrap(),
            end: second_range.next().unwrap().parse::<u32>().unwrap(),
        },
    )
}

fn is_contained(pair: (Range<u32>, Range<u32>)) -> bool {
    (pair.0.start <= pair.1.start && pair.0.end >= pair.1.end)
        || (pair.1.start <= pair.0.start && pair.1.end >= pair.0.end)
}

fn is_overlapping(pair: (Range<u32>, Range<u32>)) -> bool {
    if pair.0.start == pair.1.start || pair.0.end == pair.1.end {
        true
    } else if pair.0.start < pair.1.start {
        pair.0.end >= pair.1.start
    } else {
        pair.1.end >= pair.0.start
    }
}
