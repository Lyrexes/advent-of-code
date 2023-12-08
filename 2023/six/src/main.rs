use std::rc::Rc;

#[derive(Debug)]
struct Record {
    time: u64,
    distance: u64
}



fn main() {

    let records = parse_records(include_str!("records.input"));
    let beat_record_product = records.iter()
        .map(break_record_count)
        .product::<u64>();
    println!("{:?}", beat_record_product);

    let record = parse_record_long(include_str!("records.input"));
    let beat_record_count = break_record_count(&record);
    println!("{:?}", beat_record_count);

}

fn break_record_count(record: &Record) -> u64 {
   (1..=record.time)
       .map(|x| x * (record.time - x))
       .filter(|&x| x > record.distance)
       .count() as u64
}

fn parse_records(raw: &str) -> Vec<Record> {
    let (mut time_raw, mut distance_raw) = raw.split_once("\n").unwrap();

    time_raw = time_raw.strip_prefix("Time:").unwrap();
    time_raw = time_raw.trim();
    let time_stamps = time_raw.split_whitespace()
        .map(|x| x.parse::<u64>()
        .unwrap())
        .collect::<Rc<[u64]>>();
    
    distance_raw = distance_raw.strip_prefix("Distance:").unwrap();
    distance_raw = distance_raw.trim();
    let distances = distance_raw.split_whitespace()
        .map(|x| x.parse::<u64>()
        .unwrap())
        .collect::<Rc<[u64]>>();

    (0..time_stamps.len())
        .map(|x| Record{ time: time_stamps[x], distance: distances[x]})
        .collect()
}

fn parse_record_long(raw: &str) -> Record {
    let (mut time_raw, mut distance_raw) = raw.split_once("\n").unwrap();

    time_raw = time_raw.strip_prefix("Time:").unwrap();
    time_raw = time_raw.trim();
    let time = time_raw.chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    
    distance_raw = distance_raw.strip_prefix("Distance:").unwrap();
    distance_raw = distance_raw.trim();
    let distance = distance_raw.chars()
        .filter(|x| x.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    Record { time, distance }
}
